import {
    Vector3, SphereGeometry, Material, Mesh, WebGLRenderer, Scene,
    PerspectiveCamera, MeshNormalMaterial, Box3, AmbientLight,
} from 'three';
import Experiment from './Experiment';
import { GUI } from 'dat.gui';

export default class NBodySimulation implements Experiment {
    private running: boolean = true;
    private bodies: Body[] = [];
    private materials: Material[] = [new MeshNormalMaterial()];
    private scene: Scene = new Scene();
    private camera = new PerspectiveCamera(75);
    private ambient = new AmbientLight(0xffffff, 1.0);

    public get bodyCount(): number {
        return this.bodies.length;
    }

    public set bodyCount(n: number) {
        this.scene.remove(...this.bodies.map((body) => body.mesh));
        this.bodies = [];

        for (let i = 0; i < n; i++) {
            const radius = 1 + Math.random() * 10;
            const body = new Body(this.randomMaterial(), radius);
            body.centre.x = Math.random() * 100 - 50;
            body.centre.y = Math.random() * 100 - 50;
            body.centre.z = Math.random() * 100 - 50;
            body.velocity.x = Math.random() * 100 - 50;
            body.velocity.y = Math.random() * 100 - 50;
            body.velocity.z = Math.random() * 100 - 50;
            this.scene.add(body.mesh);

            this.bodies.push(body);
        }
    }

    public initialize(canvas: HTMLCanvasElement, renderer: WebGLRenderer, controls: GUI): void {
        this.bodyCount = 3;
        controls.add(this, 'bodyCount', 2, 100).listen();
        controls.add(this, 'running');

        this.zoomToFit();
        this.onResize(canvas);
    }

    public animate(renderer: WebGLRenderer, dt: number): void {
        renderer.render(this.scene, this.camera);
        this.updateWorld(dt);
    }

    public beforeDestroy(): void {
        this.materials.forEach((mat) => mat.dispose());
    }

    public onResize(canvas: HTMLCanvasElement): void {
        this.camera.aspect = canvas.width / canvas.height;
        this.camera.updateProjectionMatrix();
    }

    private updateWorld(dt: number): void {
        if (!this.running) {
            return;
        }
    }

    private randomMaterial(): Material {
        const ix = Math.floor(Math.random() * this.materials.length);
        return this.materials[ix];
    }

    private zoomToFit(offset: number = 1.25): void {
        // taken mostly from https://discourse.threejs.org/t/camera-zoom-to-fit-object/936/3
        const boundingBox = new Box3();

        for (const body of this.bodies) {
            boundingBox.expandByObject(body.mesh);
        }

        const centre = boundingBox.getCenter(new Vector3());
        const size = boundingBox.getSize(new Vector3());

        const maxDim = Math.max(size.x, size.y, size.z);
        const fov = this.camera.fov * (Math.PI / 180);
        const cameraZ = Math.abs(maxDim / 4 * Math.tan(fov * 2));

        this.camera.position.z = cameraZ * offset;

        const minZ = boundingBox.min.z;
        const cameraToFarEdge = (minZ < 0) ? -minZ + cameraZ : cameraZ - minZ;

        this.camera.far = cameraToFarEdge * 3;
        this.camera.updateProjectionMatrix();

        this.camera.lookAt(centre);
    }
}

// tslint:disable-next-line: max-classes-per-file
class Body {
    private static readonly DefaultRadius = 10.0;
    private static readonly SegmentCount = 10;

    public velocity: Vector3 = new Vector3();
    public acceleration: Vector3 = new Vector3();
    public mesh: Mesh;

    public constructor(material: Material, radius?: number) {

        const ball = new SphereGeometry(radius || Body.DefaultRadius, Body.SegmentCount, Body.SegmentCount);
        this.mesh = new Mesh(ball, material);
    }

    public get centre(): Vector3 {
        return this.mesh.position;
    }

    public set centre(value: Vector3) {
        this.mesh.position = value;
    }
}
