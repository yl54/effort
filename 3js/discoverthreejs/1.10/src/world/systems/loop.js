// import
import { Clock } from '../../.././vendor/three/build/three.module.js';

const clock = new Clock();

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
		// getDelta gets the difference between the last time getDelta was called.
		// calling this once at the beginning of the tick function will
		// give us an indication of how fast the update cycles.
		const delta = clock.getDelta();
		
		for (const object of this.updateables) {
			object.tick(delta);
		}
	}
}

// export
export { Loop }