import { WebGLRenderer, PerspectiveCamera, Scene } from 'three';
import Experiment from './Experiment';
import { cleanupObject3D } from './helpers';

/**
 * Help two friends find each other in the dark before they run out of light.
 *
 * Alternate between each friend to navigate through the darkness in a
 * mysterious labyrinth. Can you help them escape?
 *
 * Lost Without You is a turn-based action puzzler.
 *
 * I totally ripped off Jack Rugile from https://github.com/jackrugile/lost-without-you-ld41
 */
export default class LostWithoutYou implements Experiment {
    private BackgroundColour = 0x708090;

    private camera: PerspectiveCamera = new PerspectiveCamera(75);
    private scene: Scene = new Scene();

    public initialize(canvas: HTMLCanvasElement, renderer: WebGLRenderer): void {
        renderer.setClearColor(this.BackgroundColour);

        throw new Error("Method not implemented.");
    }

    public animate(renderer: WebGLRenderer, dt: number): void {
        renderer.render(this.scene, this.camera);

        throw new Error("Method not implemented.");
    }

    public onResize(canvas: HTMLCanvasElement): void {
        this.camera.aspect = canvas.width / canvas.height;
        this.camera.updateProjectionMatrix();
    }

    public beforeDestroy() {
        this.scene.traverse(cleanupObject3D);
    }
}
