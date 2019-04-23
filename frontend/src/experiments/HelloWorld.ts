import { GUI } from 'dat.gui';
import {
    WebGLRenderer, PerspectiveCamera, Scene, Mesh, BoxGeometry,
    MeshNormalMaterial,
} from 'three';
import Experiment from './Experiment';
import { cleanupObject3D } from './helpers';

/**
 * A basic spinning cube.
 */
export default class HelloWorld implements Experiment {
    private BackgroundColour = 0xeeeeee;

    private camera: PerspectiveCamera = new PerspectiveCamera(75);
    private scene: Scene = new Scene();
    private box?: Mesh;

    public initialize(canvas: HTMLCanvasElement, renderer: WebGLRenderer, controls: GUI): void {
        renderer.setClearColor(this.BackgroundColour);
        this.onResize(canvas);

        const geometry = new BoxGeometry(10, 10, 10);
        const material = new MeshNormalMaterial();
        this.box = new Mesh(geometry, material);
        this.scene.add(this.box);

        this.camera.position.set(0, 0, 20);
        this.camera.lookAt(this.box.position);
    }

    public animate(renderer: WebGLRenderer, dt: number): void {
        renderer.render(this.scene, this.camera);

        if (this.box) {
            this.box.rotation.x += dt;
            this.box.rotation.y += dt;
        }
    }

    public onResize(canvas: HTMLCanvasElement): void {
        this.camera.aspect = canvas.width / canvas.height;
        this.camera.updateProjectionMatrix();
    }

    public beforeDestroy() {
        this.scene.traverse(cleanupObject3D);
    }
}
