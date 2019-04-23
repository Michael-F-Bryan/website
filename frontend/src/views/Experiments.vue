<template>
    <b-container>
        <b-row align-h="center" class="my-md-3">
            <h1>{{title}}</h1>
        </b-row>

        <canvas ref="canvas"></canvas>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Experiment, { experiments, ExperimentFactory } from '../experiments/Experiment';
import HelloWorld from '../experiments/HelloWorld';
import { Dictionary } from 'vue-router/types/router';
import { WebGLRenderer, Clock } from 'three';

@Component({})
export default class Experiments extends Vue {
    private experiment: Experiment | null = null;
    private renderer: WebGLRenderer | null = null;
    private clock = new Clock();

    get slug(): string {
        return this.$route.params.slug || 'unknown';
    }

    get title(): string {
        if (this.factory) {
            return this.factory.title;
        } else {
            throw new Error(`Unknown experiment, "${this.slug}"`);
        }
    }

    public mounted() {
        this.renderer = new WebGLRenderer({ canvas: this.canvas });

        // make sure we know which experiment we're running
        if (!this.factory) {
            this.$router.push('/');
            return;
        }

        this.experiment = this.factory.create();
        this.experiment.initialize(this.canvas, this.renderer);

        window.addEventListener('resize', this.onResize);
        this.canvas.addEventListener('mousedown', this.onMouseDown);
        this.canvas.addEventListener('keypress', this.onKeyPress);
        this.renderer.setAnimationLoop(this.animate);

        this.onResize();
    }

    public beforeDestroy() {
        if (this.experiment && this.experiment.beforeDestroy) {
            this.experiment.beforeDestroy();
        }

        if (this.renderer) {
            // Hacky typecast because you can clear setAnimationLoop by passing
            // in null, but annoyingly their type definitions don't accept
            // `Function|null`...
            type Callback = () => void;
            type SetAnimationLoop = (_: Callback | null) => void;
            const sal = this.renderer.setAnimationLoop as SetAnimationLoop;
            sal(null);
        }

        window.removeEventListener('resize', this.onResize);
        this.canvas.removeEventListener('mousedown', this.onMouseDown);
        this.canvas.removeEventListener('keypress', this.onKeyPress);
    }

    private get factory(): ExperimentFactory | undefined {
        return experiments.find((f) => f.slug === this.slug);
    }

    private animate() {
        const dt = this.clock.getDelta();

        if (this.experiment && this.renderer) {
            this.experiment.animate(this.renderer, dt);
        }
    }

    private onResize() {
        this.canvas.height = Math.max(window.innerHeight - this.canvas.offsetTop - 10, 100);
        this.canvas.width = window.innerWidth - 2 * this.canvas.offsetLeft;

        if (this.renderer) {
            this.renderer.setSize(this.canvas.width, this.canvas.height);
        }

        if (this.experiment && this.experiment.onResize) {
            this.experiment.onResize(this.canvas);
        }
    }

    private onMouseDown(e: MouseEvent) {
        if (this.experiment && this.experiment.onMouseDown) {
            this.experiment.onMouseDown(e);
        }
    }

    private onKeyPress(e: KeyboardEvent) {
        if (this.experiment && this.experiment.onKeyPress) {
            this.experiment.onKeyPress(e);
        }
    }

    get canvas(): HTMLCanvasElement {
        return this.$refs.canvas as HTMLCanvasElement;
    }
}
</script>
