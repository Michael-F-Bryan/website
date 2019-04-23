import {
    WebGLRenderer, PerspectiveCamera, Scene, Mesh, MeshLambertMaterial,
    BoxGeometry, PointLight,
} from 'three';
import Experiment from './Experiment';
import { cleanupObject3D } from './helpers';

/**
 * A basic spinning cube.
 */
export default class HelloWorld implements Experiment {
    private BackgroundColour = 0xeeeeee;
    private LightColour = 0xffed4f;
    private MaterialColour = 0x2e8fff;

    private camera: PerspectiveCamera = new PerspectiveCamera(75);
    private scene: Scene = new Scene();
    private light = new PointLight(this.LightColour, 1.0);
    private box?: Mesh;

    public initialize(canvas: HTMLCanvasElement, renderer: WebGLRenderer): void {
        renderer.setClearColor(this.BackgroundColour);
        this.onResize(canvas);

        this.scene.add(this.light);

        const geometry = new BoxGeometry(10, 10, 10);
        const material = new MeshLambertMaterial({ color: this.MaterialColour });
        this.box = new Mesh(geometry, material);
        this.scene.add(this.box);

        this.light.position.set(5, 20, 20);
        this.camera.position.set(0, 0, 30);
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
