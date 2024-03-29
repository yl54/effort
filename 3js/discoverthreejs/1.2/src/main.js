import {
  BoxBufferGeometry,
  Color,
  Mesh,
  MeshBasicMaterial,
  PerspectiveCamera,
  Scene,
  WebGLRenderer,
} from '.././vendor/three/build/three.module.js';


// Get a reference to the html scene container element
const container = document.querySelector('#scene-container');

// Create a scene
const scene = new Scene();

// Set the background color
scene.background = new Color('skyblue');

// Create a camera

// frustum fields

// field of view
const fov = 35;

// aspect
const aspect = container.clientWidth / container.clientHeight;

// near clipping plane
const near = 0.1;

// far clipping plane
const far = 100;

// camera object
const camera = new PerspectiveCamera(fov, aspect, near, far);


// Position the camera behind the objects
// You could set the coordinates individually as well
camera.position.set(0, 0, 10);

// Create a geometry
// BoxBufferGeometry() = BoxBufferGeometry(1, 1, 1)
const geometry = new BoxBufferGeometry(2, 2, 2);

// Create a material
const material = new MeshBasicMaterial();

// Create a mesh from the geometry and material
const cube = new Mesh(geometry, material);

// Add the mesh to the scene
scene.add(cube);

// Create a renderer
const renderer = new WebGLRenderer();

// Set the renderer width/height
renderer.setSize(container.clientWidth, container.clientHeight);

// Set the pixel ratio of the device's screen
renderer.setPixelRatio(window.devicePixelRatio);

// Add the <canvas> element to the page
container.append(renderer.domElement);

// Render the scene
// This only creates a still image of the scene.
renderer.render(scene, camera);
