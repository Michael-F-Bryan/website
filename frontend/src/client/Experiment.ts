import {
    WebGLRenderer, PerspectiveCamera, Scene, Mesh, MeshLambertMaterial,
    BoxGeometry, PointLight, Object3D, Material, Texture, Geometry,
} from 'three';

export interface Experiment {
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


export class HelloWorld implements Experiment {
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

function cleanupObject3D(obj: Object3D) {
    if (isMesh(obj)) {
        cleanupMesh(obj);
    } else if (isDisposable(obj)) {
        obj.dispose();
    }
}

function cleanupMesh(mesh: Mesh) {
    mesh.children.forEach(cleanupObject3D);
    mesh.geometry.dispose();

    if (isMaterial(mesh.material)) {
        mesh.material.dispose();
    } else {
        mesh.material.forEach((mat) => mat.dispose());
    }
}

function isMesh(obj: any): obj is Mesh {
    return obj.isMesh;
}

function isMaterial(obj: any): obj is Material {
    return obj.isMaterial;
}

interface Disposable {
    dispose(): void;
}

function isDisposable(obj: any): obj is Disposable {
    return isCallable(obj.dispose);
}

function isCallable(obj: any): obj is () => void {
    return !!(obj && obj.constructor && obj.call && obj.apply);
}
