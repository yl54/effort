// import
import { Group, MathUtils } from '../../../.././vendor/three/build/three.module.js';

import { createMeshes } from './meshes.js';

// speed of the wheel, we want it to rotate 24 degrees 
const wheelSpeed = MathUtils.degToRad(24);

// create function
class Train extends Group {
	constructor() {
		// Call the constructor for the class it inherits from
		super();

		this.meshes = createMeshes();

		// this adds the meshes to the group
		this.add(
			this.meshes.chimney,
			this.meshes.cabin,
			this.meshes.nose,
			this.meshes.smallWheelRear,
			this.meshes.smallWheelCenter,
			this.meshes.smallWheelFront,
			this.meshes.bigWheel,
		);

		this.tick = (delta) => {
			this.meshes.smallWheelRear.rotation.y += delta * wheelSpeed;
			this.meshes.smallWheelCenter.rotation.y += delta * wheelSpeed;
			this.meshes.smallWheelFront.rotation.y += delta * wheelSpeed;
			this.meshes.bigWheel.rotation.y += delta * wheelSpeed;
		};
	}
}

// export function
export { Train };