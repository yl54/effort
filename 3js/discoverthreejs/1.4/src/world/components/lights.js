// import
import { 
	DirectionalLight, 
	PointLight, 
	SpotLight, 
	RectAreaLight 
} from '../../.././vendor/three/build/three.module.js';


// class
function createLights() {
	// Challenge Easy: 1
	// Change the color of the light
	// create the light
	// const light = new DirectionalLight('gray', 30);

	// Challenge Medium: 1
	// Change the light type
	// Point Light: a light that gets emitted from a single point in all directions.
	const light = new PointLight('white', 30);
	
	// Spot Light: a light that gets emitted from a single point in one direction, along a cone that increases the further from the light it gets.
	// const light = new SpotLight('gray', 100);

	// Rect Area Light: a light that emits light uniformly across the face a rectangular plane.
	// const light = new RectAreaLight('gray', 30);

	// Challenge Easy: 3
	// Change the position of the light
	// position the light
	// it will go:
	// * right
	// * up
	// * towards us
	// it shines from (10, 20, 5) to (0, 0, 0)
	// light.position.set(10, 20, 5);

	light.position.set(10, 10, 20);


	// return 
	return light;
}

// export
export { createLights };