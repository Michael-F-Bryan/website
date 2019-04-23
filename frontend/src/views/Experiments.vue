<template>
    <b-container>
        <b-row align-h="center" class="my-md-3">
            <h1>Experiments</h1>
        </b-row>

        <canvas ref="canvas"></canvas>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Experiment, HelloWorld } from '../client/Experiment';
import { Dictionary } from 'vue-router/types/router';
import { WebGLRenderer } from 'three';

type ExperimentConstructor = new () => Experiment;

const experiments: Dictionary<ExperimentConstructor> = {
    'hello-world': HelloWorld,
};

@Component({})
export default class Experiments extends Vue {
    private experiment?: Experiment;
    private renderer?: WebGLRenderer;
    private token?: number;
    private lastFrame: Date = new Date();

    public mounted() {
        this.renderer = new WebGLRenderer({ canvas: this.canvas });

        const name = this.$route.params.name;
        const constructor = experiments[name];

        if (constructor === undefined) {
            this.$router.push('/');
            return;
        }

        this.experiment = new constructor();
        this.experiment.initialize(this.canvas, this.renderer);


        this.token = requestAnimationFrame(this.animate);
        window.addEventListener('resize', this.onResize);
        this.canvas.addEventListener('mousedown', this.onMouseDown);
        this.canvas.addEventListener('keypress', this.onKeyPress);

        this.onResize();
    }

    public beforeDestroy() {
        if (this.experiment && this.experiment.beforeDestroy) {
            this.experiment.beforeDestroy();
        }

        if (this.token) {
            cancelAnimationFrame(this.token);
        }

        window.removeEventListener('resize', this.onResize);
        this.canvas.removeEventListener('mousedown', this.onMouseDown);
        this.canvas.removeEventListener('keypress', this.onKeyPress);
    }

    private animate() {
        const now = new Date();
        const dt = (now.valueOf() - this.lastFrame.valueOf()) / 1000;

        if (this.experiment && this.renderer) {
            this.experiment.animate(this.renderer, dt);
        }

        this.token = requestAnimationFrame(this.animate);
        this.lastFrame = now;
    }

    private onResize() {
        this.canvas.height = window.innerHeight - this.canvas.offsetTop - 10;
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

<style>
</style>
