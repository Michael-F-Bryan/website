import { WebGLRenderer } from 'three';

/**
 * Something which can be used in a WebGL experiment.
 */
export default interface Experiment {
    /**
     * Initialize the experiment.
     * @param canvas The canvas this experiment should draw to.
     * @param renderer The renderer which will be used for this experiment.
     */
    initialize(canvas: HTMLCanvasElement, renderer: WebGLRenderer): void;
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
    onMouseDown?(e: MouseEvent): void;
    onKeyPress?(e: KeyboardEvent): void;
    onResize?(canvas: HTMLCanvasElement): void;
}
