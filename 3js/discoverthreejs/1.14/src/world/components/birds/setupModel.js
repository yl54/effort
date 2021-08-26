// imports
import { AnimationMixer } from '../../../.././vendor/three/build/three.module.js';

// function
function setupModel(data) {
	// get the model from the gltf file
	const model = data.scene.children[0];

	// get the animation/clip from the gltf file
	// it could have any number of animations
	const clip = data.animations[0];

	// create an animation mixer
	const mixer = new AnimationMixer(model);

	// get the animation/clip action and play the action
	// TODO: It won't animate, its not clear why. The tick function is getting called.
	const action = mixer.clipAction(clip);
	action.play();

	// setup a tick method to have this be constantly updated
	// the loop updateables is calling tick
	model.tick = (delta) => mixer.update(delta);

	return model;
}

// exports
export { setupModel };