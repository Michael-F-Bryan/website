import { Object3D, Mesh, Material } from 'three';

export function cleanupObject3D(obj: Object3D) {
    if (isMesh(obj)) {
        cleanupMesh(obj);
    } else if (isDisposable(obj)) {
        obj.dispose();
    }
}

export function cleanupMesh(mesh: Mesh) {
    mesh.children.forEach(cleanupObject3D);
    mesh.geometry.dispose();

    if (isMaterial(mesh.material)) {
        mesh.material.dispose();
    } else {
        mesh.material.forEach((mat) => mat.dispose());
    }
}

export function isMesh(obj: any): obj is Mesh {
    return obj.isMesh;
}

export function isMaterial(obj: any): obj is Material {
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
