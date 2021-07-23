// import
import { Clock } from '../../.././vendor/three/build/three.module.js';

// class
class Loop {
	constructor(camera, scene, renderer) {
		this.camera = camera;
		this.scene = scene;
		this.renderer = renderer;
		this.updateables = [];
	}

	start() {
		this.renderer.setAnimationLoop(() => {
			this.tick();

			// render the scene
			this.renderer.render(this.scene, this.camera);
		});
	}

	stop() {
		// null callback will stop the loop
		this.renderer.setAnimationLoop(null);
	}

	// tick is the code where animations will be updated
	tick() {
		for (const object of this.updateables) {
			object.tick();
		}
	}
}

// export
export { Loop }