import {
    Vector3, SphereGeometry, Material, Mesh, WebGLRenderer, Scene,
    PerspectiveCamera, MeshNormalMaterial, Box3, AmbientLight,
} from 'three';
import Experiment from './Experiment';
import { GUI } from 'dat.gui';

export default class NBodySimulation implements Experiment {
    private static readonly InitialWorldSize: number = 1e9;

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
            body.centre = this.randomVector(NBodySimulation.InitialWorldSize);
            body.velocity = this.randomVector(NBodySimulation.InitialWorldSize);
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

        if (this.running) {
            this.updateWorld(dt);
        }

        this.zoomToFit();
    }

    public beforeDestroy(): void {
        this.materials.forEach((mat) => mat.dispose());
    }

    public onResize(canvas: HTMLCanvasElement): void {
        this.camera.aspect = canvas.width / canvas.height;
        this.camera.updateProjectionMatrix();
    }

    private randomVector(radius: number): Vector3 {
        const x = Math.random() * 2 * radius - radius;
        const y = Math.random() * 2 * radius - radius;
        const z = Math.random() * 2 * radius - radius;
        return new Vector3(x, y, z);
    }

    private updateWorld(dt: number): void {
        for (const body of this.bodies) {
            const others = this.bodies.filter((other) => other !== body);
            const forces = others.map((other) => body.gravitationalAttractionTo(other));
            const resultantForce = forces.reduce((acc, elem) => acc.add(elem), new Vector3());

            body.acceleration = resultantForce.divideScalar(body.mass);
        }

        for (const body of this.bodies) {
            body.update(dt);
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

        this.camera.position = centre;
        this.camera.position.z = cameraZ * offset;

        const minZ = boundingBox.min.z;
        const cameraToFarEdge = (minZ < 0) ? -minZ + cameraZ : cameraZ - minZ;

        this.camera.far = cameraToFarEdge * 3;
        this.camera.updateProjectionMatrix();

        this.camera.lookAt(centre);
    }
}

const G = 6.67408e-11;

// tslint:disable-next-line: max-classes-per-file
class Body {
    private static readonly DefaultRadius = 10.0;
    private static readonly SegmentCount = 10;

    public velocity: Vector3 = new Vector3();
    public acceleration: Vector3 = new Vector3();
    public mesh: Mesh;
    private radius: number;
    private readonly density: number = 1.0;

    public constructor(material: Material, radius: number = Body.DefaultRadius) {
        this.radius = radius;
        const ball = new SphereGeometry(this.radius, Body.SegmentCount, Body.SegmentCount);
        this.mesh = new Mesh(ball, material);
    }

    public get centre(): Vector3 {
        return this.mesh.position;
    }

    public set centre(value: Vector3) {
        this.mesh.position.set(value.x, value.y, value.z);
    }

    public get mass(): number {
        const volume = 4 / 3 * Math.PI * this.radius ** 3;
        return volume * this.density;
    }

    /**
     * The resultant force felt due to the other body's gravitational
     * attraction.
     * @param other The other body.
     */
    public gravitationalAttractionTo(other: Body): Vector3 {
        const direction = other.centre.sub(this.centre);
        const radius = Math.max(direction.length(), this.radius, other.radius);
        const magnitude = G * this.mass * other.mass / radius ** 2;

        return direction.divideScalar(radius).multiplyScalar(magnitude);
    }

    public update(dt: number): void {
        const deltaV = this.acceleration.multiplyScalar(dt);
        this.velocity = this.velocity.add(deltaV);

        const moved = this.velocity.multiplyScalar(dt);
        this.centre = this.centre.add(moved);
    }
}
