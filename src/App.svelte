<script lang="ts">
    import {onMount} from 'svelte';
    import {World} from '../pkg';

    const CELL_SIZE = 20;
    const GRID_COLOR = '#122334';
    const WORLD_SIZE = 8;

    let world: World;
    let canvas: HTMLCanvasElement;

    function initializeCanvas(ctx: CanvasRenderingContext2D) {
        const width = CELL_SIZE * world.width();
        const height = CELL_SIZE * world.height();

        canvas.width = width;
        canvas.height = height;

        // Clear the canvas
        ctx.clearRect(0, 0, width, height);
    }

    function drawGrid(ctx: CanvasRenderingContext2D) {
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        // Draw vertical lines
        for (let x = 0; x <= world.width(); x++) {
            ctx.moveTo(x * CELL_SIZE, 0);
            ctx.lineTo(x * CELL_SIZE, canvas.height);
        }

        // Draw horizontal lines
        for (let y = 0; y <= world.height(); y++) {
            ctx.moveTo(0, y * CELL_SIZE);
            ctx.lineTo(canvas.width, y * CELL_SIZE);
        }

        ctx.stroke();
    }

    function drawSnake(ctx: CanvasRenderingContext2D) {
        const snakeHeadIdx = world.snake_head_idx();
        const col = snakeHeadIdx % world.width();
        const row = Math.floor(snakeHeadIdx / world.width());

        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
    }

    onMount(() => {
        world = World.new(WORLD_SIZE);

        const ctx = canvas.getContext('2d');
        if (!ctx) {
            console.error('Could not get 2D context from canvas');
            return;
        }

        initializeCanvas(ctx);
        setInterval(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            drawGrid(ctx);
            drawSnake(ctx);
            world.update();
        }, 100);
    });
</script>

<canvas bind:this={canvas}></canvas>

<main>
    <div class="top-0 left-0 w-full h-full absolute flex items-center justify-center flex-col">
        <canvas
                bind:this={canvas}
        ></canvas>
    </div>
</main>
