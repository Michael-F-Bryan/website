import { GUI } from 'dat.gui';
import { WebGLRenderer } from 'three';
import HelloWorld from './HelloWorld';
import NBodySimulation from './NBodySimulation';

/**
 * Something which can be used in a WebGL experiment.
 */
export default interface Experiment {
    /**
     * Initialize the experiment.
     * @param canvas The canvas this experiment should draw to.
     * @param renderer The renderer which will be used for this experiment.
     */
    initialize(canvas: HTMLCanvasElement, renderer: WebGLRenderer, controls: GUI): void;
    /**
     * Execute any update and rendering logic.
     * @param dt The number of seconds since the last frame.
     */
    animate(renderer: WebGLRenderer, dt: number): void;

    /**
     * A callback which is fired immediately before the experiment is removed
     * from the DOM. This lets it do any necessary cleanup.
     */
    beforeDestroy?(): void;
    onMouseMove?(e: MouseEvent): void;
    onMouseDown?(e: MouseEvent): void;
    onMouseUp?(e: MouseEvent): void;
    onKeyDown?(e: KeyboardEvent): void;
    onKeyUp?(e: KeyboardEvent): void;
    onResize?(canvas: HTMLCanvasElement): void;
}

type ExperimentConstructor = new () => Experiment;

export class Factory {
    public readonly title: string;
    public readonly slug: string;
    private readonly creator: ExperimentConstructor;

    public constructor(title: string, slug: string, create: ExperimentConstructor) {
        this.title = title;
        this.slug = slug;
        this.creator = create;
    }

    public create(): Experiment {
        return new this.creator();
    }
}

export const experiments: Factory[] = [
    new Factory('Hello World', 'hello-world', HelloWorld),
    new Factory('N Body Simulation', 'n-bodies', NBodySimulation),
];
