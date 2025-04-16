"use client";
import { useEffect, useRef, useState } from "react";
import init, { Tetris } from "../../public/wasm/tetris_wasm.js";

export default function TetrisPage() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const nextCanvasRef = useRef<HTMLCanvasElement>(null);
  const [score, setScore] = useState(0);
  const [gameOver, setGameOver] = useState(false);
  const [paused, setPaused] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [clearedLanes, setClearedLanes] = useState(0);
  const [tetrisCount, setTetrisCount] = useState(0);
  const [tripleCount, setTripleCount] = useState(0);
  const [doubleCount, setDoubleCount] = useState(0);
  const [singleCount, setSingleCount] = useState(0);
  const gameRef = useRef<Tetris | null>(null);
  const lastTimeRef = useRef<number>(0);

  useEffect(() => {
    let animationFrameId: number;
    let dropInterval: NodeJS.Timeout | null = null;

    const run = async () => {
      try {
        await init("/wasm/tetris_wasm_bg.wasm");
        const game = new Tetris("game-canvas");
        gameRef.current = game;
        game.start();

        const update = (currentTime: number) => {
          const deltaTime = (currentTime - lastTimeRef.current) / 1000.0;
          lastTimeRef.current = currentTime;

          if (!game.is_game_over() && !game.is_paused()) {
            game.update_clearing_animation(deltaTime);
            game.draw();
            game.draw_next("next-canvas");
            setScore(game.get_score());
            setClearedLanes(game.get_cleared_lanes());
            setTetrisCount(game.get_tetris_count());
            setTripleCount(game.get_triple_count());
            setDoubleCount(game.get_double_count());
            setSingleCount(game.get_single_count());
          } else {
            setGameOver(game.is_game_over());
            setPaused(game.is_paused());
          }
          animationFrameId = requestAnimationFrame(update);
        };

        const startDropInterval = () => {
          if (dropInterval) clearInterval(dropInterval);
          dropInterval = setInterval(() => {
            if (gameRef.current && !gameRef.current.is_game_over() && !gameRef.current.is_paused()) {
              gameRef.current.move_down();
            }
          }, 1000);
        };

        lastTimeRef.current = performance.now();
        startDropInterval();
        update(lastTimeRef.current);

        return () => {
          cancelAnimationFrame(animationFrameId);
          if (dropInterval) clearInterval(dropInterval);
        };
      } catch (e) {
        setError(`Failed to load Tetris: ${e}`);
      }
    };

    run();

    const handleKeyDown = (e: KeyboardEvent) => {
      if (!gameRef.current || gameRef.current.is_game_over()) return;
      switch (e.key.toLowerCase()) {
        case "a":
          gameRef.current.move_left();
          break;
        case "d":
          gameRef.current.move_right();
          break;
        case "s":
          gameRef.current.move_down();
          break;
        case "w":
          gameRef.current.drop();
          break;
        case "o":
          gameRef.current.rotate_left();
          break;
        case "p":
          gameRef.current.rotate_right();
          break;
        case " ":
          if (gameRef.current.is_paused()) {
            gameRef.current.unpause();
            setPaused(false);
          } else {
            gameRef.current.pause();
            setPaused(true);
          }
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  if (error) {
    return (
      <div className="text-center text-red-500">
        <h1>Error</h1>
        <p>{error}</p>
      </div>
    );
  }

  return (
    <div className="flex justify-center items-center min-h-screen w-full">
      {gameOver && (
        <p className="absolute bg-gray-600 text-white py-2 px-4 shadow-lg">
          Game Over! Refresh to restart.
        </p>
      )}
      <div className="flex gap-5">
        <canvas
          id="game-canvas"
          className="border-2 border-gray-400"
          ref={canvasRef}
          width={300}
          height={600}
        />
        <div className="flex flex-col">
          <canvas
            id="next-canvas"
            ref={nextCanvasRef}
            width={120}
            height={120}
            className="border-2 border-gray-400"
          />
          <ul className="list-none p-0 mt-2.5 text-left font-bold">
            <li>Score: {score}</li>
            <li>Cleared: {clearedLanes}</li>
          </ul>
          <ul className="list-none p-0 mt-2.5 text-left">
            <li>Tetris: {tetrisCount}</li>
            <li>Triple: {tripleCount}</li>
            <li>Double: {doubleCount}</li>
            <li>Single: {singleCount}</li>
          </ul>
          <p className="font-bold mt-4">Controls</p>
          <ul className="list-none p-0 mt-2.5 text-left">
            <li>A - Move Left</li>
            <li>D - Move Right</li>
            <li>S - Move Down</li>
            <li>W - Drop</li>
            <li>O - Rotate Left</li>
            <li>P - Rotate Right</li>
            <li>Space - {paused ? "Unpause" : "Pause"}</li>
          </ul>
        </div>
      </div>
    </div>
  );
}