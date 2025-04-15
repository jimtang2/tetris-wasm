use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Tetris {
    board: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    current_piece: Option<Piece>,
    next_piece: Piece,
    score: u32,
    game_over: bool,
    paused: bool,
    ctx: Option<CanvasRenderingContext2d>,
    cleared_lanes: u32,
    tetris_count: u32,
    triple_count: u32,
    double_count: u32,
    single_count: u32,
    clearing_lines: Vec<usize>,
    clearing_animation_progress: f64, // 0.0 to 0.3 seconds
}

#[derive(Clone)]
struct Piece {
    shape: Vec<Vec<u8>>,
    x: i32,
    y: i32,
    color: u8,
}

#[wasm_bindgen]
impl Tetris {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Tetris {
        let document = match web_sys::window().and_then(|win| win.document()) {
            Some(doc) => doc,
            None => {
                web_sys::console::log_1(&"Failed to access window.document".into());
                return Tetris::new_fallback();
            }
        };

        let canvas = match document.get_element_by_id(canvas_id) {
            Some(elem) => match elem.dyn_into::<HtmlCanvasElement>() {
                Ok(canvas) => canvas,
                Err(_) => {
                    web_sys::console::log_1(&"Failed to cast element to HtmlCanvasElement".into());
                    return Tetris::new_fallback();
                }
            },
            None => {
                web_sys::console::log_1(&format!("Canvas element '{}' not found", canvas_id).into());
                return Tetris::new_fallback();
            }
        };

        let ctx = match canvas.get_context("2d") {
            Ok(Some(ctx)) => match ctx.dyn_into::<CanvasRenderingContext2d>() {
                Ok(ctx) => ctx,
                Err(_) => {
                    web_sys::console::log_1(&"Failed to cast context to CanvasRenderingContext2d".into());
                    return Tetris::new_fallback();
                }
            },
            Ok(None) => {
                web_sys::console::log_1(&"Failed to get 2d context".into());
                return Tetris::new_fallback();
            }
            Err(_) => {
                web_sys::console::log_1(&"Error getting canvas context".into());
                return Tetris::new_fallback();
            }
        };

        let width = 10;
        let height = 20;
        let board = vec![vec![0; width]; height];
        let shapes = vec![
            vec![vec![1, 1, 1, 1]], // I
            vec![vec![1, 1], vec![1, 1]], // O
            vec![vec![1, 1, 1], vec![0, 1, 0]], // T
            vec![vec![1, 1, 1], vec![1, 0, 0]], // L
            vec![vec![1, 1, 1], vec![0, 0, 1]], // J
            vec![vec![1, 1, 0], vec![0, 1, 1]], // S
            vec![vec![0, 1, 1], vec![1, 1, 0]], // Z
        ];

        Tetris {
            board,
            width,
            height,
            current_piece: None,
            next_piece: Tetris::create_piece(&shapes, None),
            score: 0,
            game_over: false,
            paused: false,
            ctx: Some(ctx),
            cleared_lanes: 0,
            tetris_count: 0,
            triple_count: 0,
            double_count: 0,
            single_count: 0,
            clearing_lines: Vec::new(),
            clearing_animation_progress: 0.0,
        }
    }

    fn new_fallback() -> Tetris {
        let width = 10;
        let height = 20;
        let board = vec![vec![0; width]; height];
        let shapes = vec![
            vec![vec![1, 1, 1, 1]], // I
            vec![vec![1, 1], vec![1, 1]], // O
            vec![vec![1, 1, 1], vec![0, 1, 0]], // T
            vec![vec![1, 1, 1], vec![1, 0, 0]], // L
            vec![vec![1, 1, 1], vec![0, 0, 1]], // J
            vec![vec![1, 1, 0], vec![0, 1, 1]], // S
            vec![vec![0, 1, 1], vec![1, 1, 0]], // Z
        ];

        Tetris {
            board,
            width,
            height,
            current_piece: None,
            next_piece: Tetris::create_piece(&shapes, None),
            score: 0,
            game_over: false,
            paused: false,
            ctx: None,
            cleared_lanes: 0,
            tetris_count: 0,
            triple_count: 0,
            double_count: 0,
            single_count: 0,
            clearing_lines: Vec::new(),
            clearing_animation_progress: 0.0,
        }
    }

    fn create_piece(shapes: &[Vec<Vec<u8>>], test_idx: Option<usize>) -> Piece {
        let idx = test_idx.unwrap_or_else(|| {
            #[cfg(target_arch = "wasm32")]
            {
                (js_sys::Math::random() * shapes.len() as f64).floor() as usize
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                0 // Default to first shape for tests
            }
        });
        let color = ((idx % 6) + 1) as u8; // Maps to 1-6 (colors #f00 to #0ff)
        Piece {
            shape: shapes[idx].clone(),
            x: 4 - shapes[idx][0].len() as i32 / 2,
            y: 0,
            color,
        }
    }

    pub fn start(&mut self) {
        if self.current_piece.is_none() {
            self.current_piece = Some(self.next_piece.clone());
            self.next_piece = Tetris::create_piece(&[
                vec![vec![1, 1, 1, 1]],
                vec![vec![1, 1], vec![1, 1]],
                vec![vec![1, 1, 1], vec![0, 1, 0]],
                vec![vec![1, 1, 1], vec![1, 0, 0]],
                vec![vec![1, 1, 1], vec![0, 0, 1]],
                vec![vec![1, 1, 0], vec![0, 1, 1]],
                vec![vec![0, 1, 1], vec![1, 1, 0]],
            ], None);
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn move_left(&mut self) {
        if self.paused {
            return;
        }
        if let Some(ref mut piece) = self.current_piece {
            let old_x = piece.x;
            piece.x -= 1;
            if collides(piece, &self.board, self.width, self.height) {
                piece.x = old_x;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.paused {
            return;
        }
        if let Some(ref mut piece) = self.current_piece {
            let old_x = piece.x;
            piece.x += 1;
            if collides(piece, &self.board, self.width, self.height) {
                piece.x = old_x;
            }
        }
    }

    pub fn move_down(&mut self) -> bool {
        if self.paused {
            return true;
        }
        if !self.clearing_lines.is_empty() {
            return true; // Wait for animation to finish
        }
        if let Some(ref mut piece) = self.current_piece {
            let old_y = piece.y;
            piece.y += 1;
            if collides(piece, &self.board, self.width, self.height) {
                piece.y = old_y;
                self.merge();
                self.clear_lines();
                self.current_piece = Some(self.next_piece.clone());
                self.next_piece = Tetris::create_piece(&[
                    vec![vec![1, 1, 1, 1]],
                    vec![vec![1, 1], vec![1, 1]],
                    vec![vec![1, 1, 1], vec![0, 1, 0]],
                    vec![vec![1, 1, 1], vec![1, 0, 0]],
                    vec![vec![1, 1, 1], vec![0, 0, 1]],
                    vec![vec![1, 1, 0], vec![0, 1, 1]],
                    vec![vec![0, 1, 1], vec![1, 1, 0]],
                ], None);
                if collides(&self.current_piece.as_ref().unwrap(), &self.board, self.width, self.height) {
                    self.game_over = true;
                    return false;
                }
            }
        }
        true
    }

    pub fn drop(&mut self) {
        if self.paused {
            return;
        }
        if !self.clearing_lines.is_empty() {
            return; // Wait for animation to finish
        }
        if let Some(ref mut piece) = self.current_piece {
            let mut temp_y = piece.y;
            while !collides(&Piece { y: temp_y + 1, ..piece.clone() }, &self.board, self.width, self.height) {
                temp_y += 1;
            }
            piece.y = temp_y;
            self.merge();
            self.clear_lines();
            self.current_piece = Some(self.next_piece.clone());
            self.next_piece = Tetris::create_piece(&[
                vec![vec![1, 1, 1, 1]],
                vec![vec![1, 1], vec![1, 1]],
                vec![vec![1, 1, 1], vec![0, 1, 0]],
                vec![vec![1, 1, 1], vec![1, 0, 0]],
                vec![vec![1, 1, 1], vec![0, 0, 1]],
                vec![vec![1, 1, 0], vec![0, 1, 1]],
                vec![vec![0, 1, 1], vec![1, 1, 0]],
            ], None);
            if collides(&self.current_piece.as_ref().unwrap(), &self.board, self.width, self.height) {
                self.game_over = true;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        if self.paused {
            return;
        }
        if !self.clearing_lines.is_empty() {
            return; // Wait for animation to finish
        }
        if let Some(ref mut piece) = self.current_piece {
            let new_shape = rotate(&piece.shape, -1);
            let old_shape = piece.shape.clone();
            piece.shape = new_shape;
            if collides(piece, &self.board, self.width, self.height) {
                piece.shape = old_shape;
            }
        }
    }

    pub fn rotate_right(&mut self) {
        if self.paused {
            return;
        }
        if !self.clearing_lines.is_empty() {
            return; // Wait for animation to finish
        }
        if let Some(ref mut piece) = self.current_piece {
            let new_shape = rotate(&piece.shape, 1);
            let old_shape = piece.shape.clone();
            piece.shape = new_shape;
            if collides(piece, &self.board, self.width, self.height) {
                piece.shape = old_shape;
            }
        }
    }

    pub fn update_clearing_animation(&mut self, delta_time: f64) {
        if self.clearing_lines.is_empty() {
            return;
        }
        self.clearing_animation_progress += delta_time;
        if self.clearing_animation_progress >= 0.3 {
            // Finish clearing
            let mut new_board: Vec<Vec<u8>> = Vec::new();
            for y in 0..self.height {
                if !self.clearing_lines.contains(&y) {
                    new_board.push(self.board[y].clone());
                }
            }
            for _ in 0..self.clearing_lines.len() {
                new_board.insert(0, vec![0; self.width]);
            }
            self.board = new_board;
            self.clearing_lines.clear();
            self.clearing_animation_progress = 0.0;
        }
    }

    fn merge(&mut self) {
        if let Some(ref piece) = self.current_piece {
            if piece.color == 0 || piece.color as usize >= 7 {
                web_sys::console::log_1(&format!("Invalid color during merge: {}", piece.color).into());
                return;
            }
            for y in 0..piece.shape.len() {
                for x in 0..piece.shape[y].len() {
                    if piece.shape[y][x] != 0 {
                        let board_y = piece.y + y as i32;
                        if board_y >= 0 && board_y < self.height as i32 {
                            self.board[board_y as usize][piece.x as usize + x] = piece.color;
                        }
                    }
                }
            }
        }
    }

    fn clear_lines(&mut self) {
        if !self.clearing_lines.is_empty() {
            return; // Already clearing
        }
        let mut lines_to_clear = Vec::new();
        let mut lines_cleared = 0;
        for y in 0..self.height {
            if self.board[y].iter().all(|&cell| cell != 0) {
                lines_to_clear.push(y);
                lines_cleared += 1;
            }
        }
        if !lines_to_clear.is_empty() {
            self.clearing_lines = lines_to_clear;
            self.clearing_animation_progress = 0.0;
            self.cleared_lanes += lines_cleared;

            match lines_cleared {
                4 => {
                    self.tetris_count += 1;
                    self.score += 1000;
                }
                3 => {
                    self.triple_count += 1;
                    self.score += 600;
                }
                2 => {
                    self.double_count += 1;
                    self.score += 300;
                }
                1 => {
                    self.single_count += 1;
                    self.score += 100;
                }
                _ => {}
            }
        }
    }

    #[allow(deprecated)]
    fn draw_background(&self, ctx: &CanvasRenderingContext2d, width: f64, height: f64, block_size: f64) {
        let grid_color = "#1C2526"; // Dark gray background
        let line_color = "#2A3435"; // Lighter gray grid lines

        // Fill the canvas with the base color
        ctx.set_fill_style(&JsValue::from_str(grid_color));
        ctx.fill_rect(0.0, 0.0, width, height);

        // Draw grid lines using stroke for precision
        ctx.set_stroke_style(&JsValue::from_str(line_color));
        ctx.set_line_width(1.0);

        // Vertical lines
        for x in 0..=(width as i32 / block_size as i32) {
            let x_pos = x as f64 * block_size;
            ctx.begin_path();
            ctx.move_to(x_pos, 0.0);
            ctx.line_to(x_pos, height);
            ctx.stroke();
        }

        // Horizontal lines
        for y in 0..=(height as i32 / block_size as i32) {
            let y_pos = y as f64 * block_size;
            ctx.begin_path();
            ctx.move_to(0.0, y_pos);
            ctx.line_to(width, y_pos);
            ctx.stroke();
        }
    }

    #[allow(deprecated)]
    pub fn draw(&self) {
        if let Some(ctx) = &self.ctx {
            let block_size = 30.0;
            let colors = vec![
                "#000", "#ff5555", "#55ff55", "#5555ff", "#ffff55", "#ff55ff", "#55ffff",
            ];
            let highlight_colors = vec![
                "#000", "#ff9999", "#99ff99", "#9999ff", "#ffff99", "#ff99ff", "#99ffff",
            ];

            // Draw light grey border
            ctx.set_fill_style(&JsValue::from_str("#d3d3d3"));
            ctx.fill_rect(-2.0, -2.0, block_size * self.width as f64 + 4.0, block_size * self.height as f64 + 4.0);

            // Replace solid black fill with grid background
            self.draw_background(&ctx, block_size * self.width as f64, block_size * self.height as f64, block_size);

            // Draw board
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.board[y][x] != 0 {
                        let color_idx = self.board[y][x] as usize;
                        if color_idx >= colors.len() || color_idx == 0 {
                            web_sys::console::log_1(&format!("Invalid color index in board: {}", color_idx).into());
                            return;
                        }
                        let alpha = if self.clearing_lines.contains(&y) {
                            1.0 - (self.clearing_animation_progress / 0.3) // Fade out
                        } else {
                            1.0
                        };
                        let gradient = ctx.create_linear_gradient(
                            x as f64 * block_size,
                            y as f64 * block_size,
                            (x as f64 + 1.0) * block_size,
                            (y as f64 + 1.0) * block_size,
                        );
                        let _ = gradient.add_color_stop(0.0, &format!("rgba({}, {}, {}, {})", 
                            u8::from_str_radix(&colors[color_idx][1..3], 16).unwrap(), 
                            u8::from_str_radix(&colors[color_idx][3..5], 16).unwrap(), 
                            u8::from_str_radix(&colors[color_idx][5..7], 16).unwrap(), 
                            alpha));
                        let _ = gradient.add_color_stop(1.0, &format!("rgba(0, 0, 0, {})", alpha));
                        ctx.set_fill_style(&gradient);
                        ctx.fill_rect(
                            x as f64 * block_size + 2.0,
                            y as f64 * block_size + 2.0,
                            block_size - 4.0,
                            block_size - 4.0,
                        );
                        if !self.clearing_lines.contains(&y) || self.clearing_animation_progress < 0.15 {
                            ctx.set_fill_style(&JsValue::from_str(&format!("rgba({}, {}, {}, {})", 
                                u8::from_str_radix(&highlight_colors[color_idx][1..3], 16).unwrap(), 
                                u8::from_str_radix(&highlight_colors[color_idx][3..5], 16).unwrap(), 
                                u8::from_str_radix(&highlight_colors[color_idx][5..7], 16).unwrap(), 
                                alpha)));
                            ctx.fill_rect(
                                x as f64 * block_size + 4.0,
                                y as f64 * block_size + 4.0,
                                block_size - 8.0,
                                block_size - 8.0,
                            );
                        }
                    }
                }
            }

            // Draw current piece
            if let Some(ref piece) = self.current_piece {
                for y in 0..piece.shape.len() {
                    for x in 0..piece.shape[y].len() {
                        if piece.shape[y][x] != 0 {
                            let color_idx = piece.color as usize;
                            if color_idx >= colors.len() || color_idx == 0 {
                                web_sys::console::log_1(&format!("Invalid color index in piece: {}", color_idx).into());
                                return;
                            }
                            let gradient = ctx.create_linear_gradient(
                                (piece.x + x as i32) as f64 * block_size,
                                (piece.y + y as i32) as f64 * block_size,
                                (piece.x + x as i32 + 1) as f64 * block_size,
                                (piece.y + y as i32 + 1) as f64 * block_size,
                            );
                            let _ = gradient.add_color_stop(0.0, colors[color_idx]);
                            let _ = gradient.add_color_stop(1.0, "#000");
                            ctx.set_fill_style(&gradient);
                            ctx.fill_rect(
                                (piece.x + x as i32) as f64 * block_size + 2.0,
                                (piece.y + y as i32) as f64 * block_size + 2.0,
                                block_size - 4.0,
                                block_size - 4.0,
                            );
                            ctx.set_fill_style(&JsValue::from_str(highlight_colors[color_idx]));
                            ctx.fill_rect(
                                (piece.x + x as i32) as f64 * block_size + 4.0,
                                (piece.y + y as i32) as f64 * block_size + 4.0,
                                block_size - 8.0,
                                block_size - 8.0,
                            );
                        }
                    }
                }
            }

            // Draw pause overlay if paused
            if self.paused {
                ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.7)"));
                ctx.fill_rect(0.0, 0.0, block_size * self.width as f64, block_size * self.height as f64);
                ctx.set_fill_style(&JsValue::from_str("#FFD700")); // Gold color for "PAUSE"
                ctx.set_font("40px Arial");
                ctx.set_text_align("center");
                let _ = ctx.fill_text("PAUSE", block_size * self.width as f64 / 2.0, block_size * self.height as f64 / 2.0);
            }
        }
    }

    #[allow(deprecated)]
    pub fn draw_next(&self, canvas_id: &str) {
        let block_size = 30.0;
        let colors = vec![
            "#000", "#ff5555", "#55ff55", "#5555ff", "#ffff55", "#ff55ff", "#55ffff",
        ];
        let highlight_colors = vec![
            "#000", "#ff9999", "#99ff99", "#9999ff", "#ffff99", "#ff99ff", "#99ffff",
        ];

        let document = match web_sys::window().and_then(|win| win.document()) {
            Some(doc) => doc,
            None => {
                web_sys::console::log_1(&"Failed to access window.document".into());
                return;
            }
        };

        let canvas = match document.get_element_by_id(canvas_id) {
            Some(elem) => match elem.dyn_into::<HtmlCanvasElement>() {
                Ok(canvas) => canvas,
                Err(_) => {
                    web_sys::console::log_1(&"Failed to cast element to HtmlCanvasElement".into());
                    return;
                }
            },
            None => {
                web_sys::console::log_1(&format!("Canvas element '{}' not found", canvas_id).into());
                return;
            }
        };

        let ctx = match canvas.get_context("2d") {
            Ok(Some(ctx)) => match ctx.dyn_into::<CanvasRenderingContext2d>() {
                Ok(ctx) => ctx,
                Err(_) => {
                    web_sys::console::log_1(&"Failed to cast context to CanvasRenderingContext2d".into());
                    return;
                }
            },
            Ok(None) => {
                web_sys::console::log_1(&"Failed to get 2d context".into());
                return;
            }
            Err(_) => {
                web_sys::console::log_1(&"Error getting canvas context".into());
                return;
            }
        };

        // Draw light grey border
        ctx.set_fill_style(&JsValue::from_str("#d3d3d3"));
        ctx.fill_rect(-2.0, -2.0, block_size * 4.0 + 4.0, block_size * 4.0 + 4.0);

        // Replace solid black fill with grid background
        self.draw_background(&ctx, block_size * 4.0, block_size * 4.0, block_size);

        // Draw the next piece
        for y in 0..self.next_piece.shape.len() {
            for x in 0..self.next_piece.shape[y].len() {
                if self.next_piece.shape[y][x] != 0 {
                    let color_idx = self.next_piece.color as usize;
                    if color_idx >= colors.len() || color_idx == 0 {
                        web_sys::console::log_1(&format!("Invalid color index in draw_next: {}", color_idx).into());
                        return;
                    }
                    let gradient = ctx.create_linear_gradient(
                        x as f64 * block_size,
                        y as f64 * block_size,
                        (x as f64 + 1.0) * block_size,
                        (y as f64 + 1.0) * block_size,
                    );
                    let _ = gradient.add_color_stop(0.0, colors[color_idx]);
                    let _ = gradient.add_color_stop(1.0, "#000");
                    ctx.set_fill_style(&gradient);
                    ctx.fill_rect(
                        x as f64 * block_size + 2.0,
                        y as f64 * block_size + 2.0,
                        block_size - 4.0,
                        block_size - 4.0,
                    );
                    ctx.set_fill_style(&JsValue::from_str(highlight_colors[color_idx]));
                    ctx.fill_rect(
                        x as f64 * block_size + 4.0,
                        y as f64 * block_size + 4.0,
                        block_size - 8.0,
                        block_size - 8.0,
                    );
                }
            }
        }
    }
    
    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_cleared_lanes(&self) -> u32 {
        self.cleared_lanes
    }

    pub fn get_tetris_count(&self) -> u32 {
        self.tetris_count
    }

    pub fn get_triple_count(&self) -> u32 {
        self.triple_count
    }

    pub fn get_double_count(&self) -> u32 {
        self.double_count
    }

    pub fn get_single_count(&self) -> u32 {
        self.single_count
    }
}

fn rotate(shape: &Vec<Vec<u8>>, direction: i32) -> Vec<Vec<u8>> {
    let mut new_shape = vec![vec![0; shape.len()]; shape[0].len()];
    for y in 0..shape.len() {
        for x in 0..shape[y].len() {
            if direction == 1 {
                new_shape[x][shape.len() - 1 - y] = shape[y][x];
            } else {
                new_shape[shape[0].len() - 1 - x][y] = shape[y][x];
            }
        }
    }
    new_shape
}

fn collides(piece: &Piece, board: &[Vec<u8>], width: usize, height: usize) -> bool {
    for y in 0..piece.shape.len() {
        for x in 0..piece.shape[y].len() {
            if piece.shape[y][x] != 0 {
                let board_x = piece.x + x as i32;
                let board_y = piece.y + y as i32;
                if board_x < 0
                    || board_x >= width as i32
                    || board_y >= height as i32
                    || (board_y >= 0 && board[board_y as usize][board_x as usize] != 0)
                {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tetris() -> Tetris {
        let width = 10;
        let height = 20;
        let board = vec![vec![0; width]; height];
        let shapes = vec![
            vec![vec![1, 1, 1, 1]], // I
            vec![vec![1, 1], vec![1, 1]], // O
            vec![vec![1, 1, 1], vec![0, 1, 0]], // T
        ];
        Tetris {
            board,
            width,
            height,
            current_piece: None,
            next_piece: Tetris::create_piece(&shapes, Some(0)),
            score: 0,
            game_over: false,
            paused: false,
            ctx: None,
            cleared_lanes: 0,
            tetris_count: 0,
            triple_count: 0,
            double_count: 0,
            single_count: 0,
            clearing_lines: Vec::new(),
            clearing_animation_progress: 0.0,
        }
    }

    #[test]
    fn test_create_piece() {
        let shapes = vec![vec![vec![1, 1, 1, 1]]];
        let piece = Tetris::create_piece(&shapes, Some(0));
        assert_eq!(piece.shape, vec![vec![1, 1, 1, 1]]);
        assert_eq!(piece.x, 2);
        assert_eq!(piece.y, 0);
        assert_eq!(piece.color, 1);
    }

    #[test]
    fn test_move_left() {
        let mut tetris = setup_tetris();
        tetris.current_piece = Some(Piece {
            shape: vec![vec![1, 1]],
            x: 4,
            y: 0,
            color: 1,
        });
        tetris.move_left();
        assert_eq!(tetris.current_piece.as_ref().unwrap().x, 3);
        for _ in 0..3 {
            tetris.move_left();
        }
        assert_eq!(tetris.current_piece.as_ref().unwrap().x, 0);
        tetris.move_left();
        assert_eq!(tetris.current_piece.as_ref().unwrap().x, 0);
    }

    #[test]
    fn test_move_down_and_merge() {
        let mut tetris = setup_tetris();
        tetris.current_piece = Some(Piece {
            shape: vec![vec![1, 1]],
            x: 4,
            y: 18,
            color: 1,
        });
        assert!(tetris.move_down());
        assert_eq!(tetris.current_piece.as_ref().unwrap().y, 19);
        assert!(tetris.move_down());
        assert!(tetris.current_piece.is_some());
        assert_eq!(tetris.board[19][4], 1);
        assert_eq!(tetris.board[19][5], 1);
    }

    #[test]
    fn test_rotate_right() {
        let mut tetris = setup_tetris();
        tetris.current_piece = Some(Piece {
            shape: vec![vec![1, 1, 1], vec![0, 1, 0]],
            x: 4,
            y: 0,
            color: 1,
        });
        tetris.rotate_right();
        let expected_shape = vec![vec![0, 1], vec![1, 1], vec![0, 1]];
        assert_eq!(tetris.current_piece.as_ref().unwrap().shape, expected_shape);
    }

    #[test]
    fn test_clear_lines() {
        let mut tetris = setup_tetris();
        // Clear 1 line
        tetris.board[19] = vec![1; 10];
        tetris.clear_lines();
        assert_eq!(tetris.clearing_lines, vec![19]);
        assert_eq!(tetris.score, 100);
        assert_eq!(tetris.cleared_lanes, 1);
        assert_eq!(tetris.single_count, 1);
        assert_eq!(tetris.double_count, 0);
        assert_eq!(tetris.triple_count, 0);
        assert_eq!(tetris.tetris_count, 0);

        // Finish animation
        tetris.update_clearing_animation(0.3);
        assert!(tetris.clearing_lines.is_empty());
        assert_eq!(tetris.board[19], vec![0; 10]);
        assert_eq!(tetris.board[0], vec![0; 10]);
    }

    #[test]
    fn test_game_over() {
        let mut tetris = setup_tetris();
        for i in 0..8 {
            tetris.board[0][i] = 1;
        }
        tetris.current_piece = Some(Piece {
            shape: vec![vec![1, 1]],
            x: 8,
            y: 0,
            color: 1,
        });
        tetris.drop();
        assert!(tetris.is_game_over());
    }
}