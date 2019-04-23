<template>
    <b-container>
        <b-row align-h="center" class="my-md-3">
            <h1>{{title}}</h1>
        </b-row>

        <b-row>
            <canvas ref="canvas"></canvas>
        </b-row>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { GUI } from 'dat.gui';
import Stats from 'stats.js';
import Experiment, { experiments, Factory } from '../experiments/Experiment';
import HelloWorld from '../experiments/HelloWorld';
import { WebGLRenderer, Clock } from 'three';

@Component({})
export default class Experiments extends Vue {
    public showStats: boolean = true;
    private experiment: Experiment | null = null;
    private renderer: WebGLRenderer | null = null;
    private clock = new Clock();
    private stats = new Stats();
    private controls = new GUI({ autoPlace: false });

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

    get canvas(): HTMLCanvasElement {
        return this.$refs.canvas as HTMLCanvasElement;
    }

    private get factory(): Factory | undefined {
        return experiments.find((f) => f.slug === this.slug);
    }

    public mounted() {
        this.renderer = new WebGLRenderer({ canvas: this.canvas });

        this.setupOverlays();

        // make sure we know which experiment we're running
        if (!this.factory) {
            this.$router.push('/');
            return;
        }

        this.experiment = this.factory.create();
        this.experiment.initialize(this.canvas, this.renderer, this.controls);

        window.addEventListener('resize', this.onResize);
        this.canvas.addEventListener('mousedown', this.onMouseDown);
        this.canvas.addEventListener('mouseup', this.onMouseUp);
        this.canvas.addEventListener('keydown', this.onKeyDown);
        this.canvas.addEventListener('keyup', this.onKeyUp);
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
        this.canvas.removeEventListener('mouseup', this.onMouseUp);
        this.canvas.removeEventListener('keydown', this.onKeyDown);
        this.canvas.removeEventListener('keyup', this.onKeyUp);

        this.controls.destroy();
    }

    private setupOverlays() {
        this.controls.add(this, 'showStats').listen();

        if (this.canvas.parentElement) {
            this.stats.showPanel(0);
            this.canvas.parentElement.appendChild(this.stats.dom);
            this.canvas.parentElement.appendChild(this.controls.domElement);
        }
    }

    private animate() {
        this.stats.begin();
        const dt = this.clock.getDelta();

        if (this.experiment && this.renderer) {
            this.experiment.animate(this.renderer, dt);
        }
        this.stats.end();
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

        this.stats.dom.style.top = this.canvas.offsetTop + 'px';
        this.stats.dom.style.left = this.canvas.offsetLeft + 'px';

        this.controls.domElement.style.position = 'fixed';
        this.controls.domElement.style.top = this.canvas.offsetTop + 'px';
        const right = this.canvas.offsetLeft + this.canvas.width - this.controls.width;
        this.controls.domElement.style.left = right + 'px';
        (this.controls.domElement as any).style['z-index'] = 10000;
    }

    private onMouseDown(e: MouseEvent) {
        if (this.experiment && this.experiment.onMouseDown) {
            this.experiment.onMouseDown(e);
        }
    }

    private onMouseUp(e: MouseEvent) {
        if (this.experiment && this.experiment.onMouseUp) {
            this.experiment.onMouseUp(e);
        }
    }

    private onKeyDown(e: KeyboardEvent) {
        if (this.experiment && this.experiment.onKeyDown) {
            this.experiment.onKeyDown(e);
        }
    }

    private onKeyUp(e: KeyboardEvent) {
        if (this.experiment && this.experiment.onKeyUp) {
            this.experiment.onKeyUp(e);
        }
    }
}
</script>
