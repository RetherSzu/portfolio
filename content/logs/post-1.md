---
id: 1
title: 01_Architecture_of_Permanence.cpp
description: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer vestibulum facilisis purus, id fringilla arcu sollicitudin ac. Vivamus ornare placerat enim, vitae pharetra tellus posuere a. Donec porttitor, justo eu convallis maximus, neque ligula tincidunt libero, sed vehicula sem ipsum eleifend quam. Suspendisse velit nisl, molestie ac justo id, eleifend pharetra dolor. Vestibulum id pellentesque odio. Sed elementum eleifend ante id eleifend. Suspendisse magna lacus, mollis nec odio vitae, dignissim scelerisque metus. Nullam facilisis at enim ac viverra. Maecenas euismod hendrerit nibh et imperdiet. Donec faucibus at libero quis consectetur. Vestibulum posuere orci eu elit elementum, vel sodales odio volutpat. Duis elit leo, mollis non tortor non, semper convallis ex.
date: 2026-03-15
---

<canvas id="glcanvas" class="w-full h-auto" style="aspect-ratio: 16 / 9;"></canvas>

<script>
    main();
    function main() {
        const canvas = document.querySelector("#glcanvas");
        const gl = canvas.getContext("webgl");

        if (!gl) {
            alert("Unable to initialize WebGL. Your browser or machine may not support it.");
            return;
        }

        // Set canvas resolution to match display size
        canvas.width = canvas.clientWidth;
        canvas.height = canvas.clientHeight;
        gl.viewport(0, 0, canvas.width, canvas.height);

        // --- Shader Source Code ---
        // Shaders are programs that run on the GPU.

        // Vertex Shader: Determines the position of each vertex (point).
        const vsSource = `
            attribute vec4 aVertexPosition;
            uniform mat4 uModelViewMatrix;
            uniform mat4 uProjectionMatrix;
            void main() {
                gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
            }
        `;

        // Fragment Shader: Determines the color of each pixel.
        const fsSource = `
            void main() {
                gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0); // Wireframe color (white)
            }
        `;

        // --- Initialize Shaders ---
        const shaderProgram = initShaderProgram(gl, vsSource, fsSource);

        const programInfo = {
            program: shaderProgram,
            attribLocations: {
                vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
            },
            uniformLocations: {
                projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
                modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
            },
        };

        // --- Define the Cube Data ---
        const buffers = initBuffers(gl);

        let cubeRotation = 0.0;
        let then = 0;

        // --- Draw Loop ---
        function render(now) {
            now *= 0.001; // convert to seconds
            const deltaTime = now - then;
            then = now;

            drawScene(gl, programInfo, buffers, cubeRotation);
            cubeRotation += deltaTime; // Increase rotation over time

            requestAnimationFrame(render);
        }
        requestAnimationFrame(render);
    }

    // --- Helper Functions ---

    // Define vertices and line indices for the wireframe cube
    function initBuffers(gl) {
        // Define the 8 corners of the cube
        const positions = [
            // Front face
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,

            // Back face
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,
        ];

        const positionBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

        // Define which vertices form each edge (line segment)
        const indices = [
            0, 1, 1, 2, 2, 3, 3, 0, // front
            4, 5, 5, 6, 6, 7, 7, 4, // back
            0, 4, 1, 7, 2, 6, 3, 5  // connecting lines
        ];

        const indexBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW);

        return {
            position: positionBuffer,
            indices: indexBuffer,
        };
    }

    // Load and compile shader source, linking them into a program
    function initShaderProgram(gl, vsSource, fsSource) {
        const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
        const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

        const shaderProgram = gl.createProgram();
        gl.attachShader(shaderProgram, vertexShader);
        gl.attachShader(shaderProgram, fragmentShader);
        gl.linkProgram(shaderProgram);

        if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
            alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
            return null;
        }

        return shaderProgram;
    }

    // Create a shader of the given type, upload source, and compile
    function loadShader(gl, type, source) {
        const shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);

        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            alert('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
            gl.deleteShader(shader);
            return null;
        }

        return shader;
    }

    // Render the scene: set matrices and draw line segments
    function drawScene(gl, programInfo, buffers, cubeRotation) {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);  // Clear to black, fully opaque
        gl.clearDepth(1.0);                 // Clear everything
        gl.enable(gl.DEPTH_TEST);           // Enable depth testing
        gl.depthFunc(gl.LEQUAL);            // Near things obscure far things

        // Clear the canvas before we start drawing on it.
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        // --- Matrix Math Setup ---
        // (For simplicity, manually defined for this example, usually a library like glMatrix is used)

        // Perspective Matrix (simulates 3D depth)
        const fieldOfView = 45 * Math.PI / 180;   // in radians
        const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
        const zNear = 0.1;
        const zFar = 100.0;
        const projectionMatrix = mat4Perspective(fieldOfView, aspect, zNear, zFar);

        // ModelView Matrix (moves and rotates the object)
        const modelViewMatrix = mat4Identity();
        mat4Translate(modelViewMatrix, [-0.0, 0.0, -6.0]); // Move the cube back 6 units
        mat4Rotate(modelViewMatrix, cubeRotation, [0, 0, 1]);    // Rotate around Z axis
        mat4Rotate(modelViewMatrix, cubeRotation * 0.7, [0, 1, 0]); // Rotate around Y axis

        // --- Point WebGL to our Data ---

        // Vertices
        {
            const numComponents = 3;  // pull out 3 values per iteration (x, y, z)
            const type = gl.FLOAT;    // the data in the buffer is 32bit floats
            const normalize = false;  // don't normalize
            const stride = 0;         // how many bytes to get from one set of values to the next
            const offset = 0;         // how many bytes inside the buffer to start from
            gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
            gl.vertexAttribPointer(
                programInfo.attribLocations.vertexPosition,
                numComponents,
                type,
                normalize,
                stride,
                offset);
            gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition);
        }

        // Indices
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffers.indices);

        // Tell WebGL to use our program
        gl.useProgram(programInfo.program);

        // Set the shader uniforms
        gl.uniformMatrix4fv(
            programInfo.uniformLocations.projectionMatrix,
            false,
            projectionMatrix);
        gl.uniformMatrix4fv(
            programInfo.uniformLocations.modelViewMatrix,
            false,
            modelViewMatrix);

        // --- EXECUTE THE DRAW! ---
        {
            const vertexCount = 24; // Number of indices defined in initBuffers
            const type = gl.UNSIGNED_SHORT; // Type of data in index buffer
            const offset = 0; // Stride offset
            // Draw as gl.LINES to create the wireframe effect
            gl.drawElements(gl.LINES, vertexCount, type, offset);
        }
    }

    // --- Minimal Matrix Math Utilities (Inlined for standalone example) ---

    function mat4Identity() {
        return new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
    }

    function mat4Perspective(fovy, aspect, near, far) {
        const f = 1.0 / Math.tan(fovy / 2);
        const nf = 1 / (near - far);
        return new Float32Array([
            f / aspect, 0, 0, 0,
            0, f, 0, 0,
            0, 0, (far + near) * nf, -1,
            0, 0, (2 * far * near) * nf, 0
        ]);
    }

    function mat4Translate(m, v) {
        m[12] = m[0] * v[0] + m[4] * v[1] + m[8] * v[2] + m[12];
        m[13] = m[1] * v[0] + m[5] * v[1] + m[9] * v[2] + m[13];
        m[14] = m[2] * v[0] + m[6] * v[1] + m[10] * v[2] + m[14];
        m[15] = m[3] * v[0] + m[7] * v[1] + m[11] * v[2] + m[15];
    }

    function mat4Rotate(m, rad, axis) {
        let x = axis[0], y = axis[1], z = axis[2];
        let len = Math.sqrt(x * x + y * y + z * z);
        if (Math.abs(len) < 0.000001) { return null; }
        len = 1 / len;
        x *= len; y *= len; z *= len;
        const s = Math.sin(rad), c = Math.cos(rad);
        const t = 1 - c;
        const a00 = m[0], a01 = m[1], a02 = m[2], a03 = m[3];
        const a10 = m[4], a11 = m[5], a12 = m[6], a13 = m[7];
        const a20 = m[8], a21 = m[9], a22 = m[10], a23 = m[11];
        const b00 = x * x * t + c, b01 = y * x * t + z * s, b02 = z * x * t - y * s;
        const b10 = x * y * t - z * s, b11 = y * y * t + c, b12 = z * y * t + x * s;
        const b20 = x * z * t + y * s, b21 = y * z * t - x * s, b22 = z * z * t + c;
        m[0] = a00 * b00 + a10 * b01 + a20 * b02; m[1] = a01 * b00 + a11 * b01 + a21 * b02;
        m[2] = a02 * b00 + a12 * b01 + a22 * b02; m[3] = a03 * b00 + a13 * b01 + a23 * b02;
        m[4] = a00 * b10 + a10 * b11 + a20 * b12; m[5] = a01 * b10 + a11 * b11 + a21 * b12;
        m[6] = a02 * b10 + a12 * b11 + a22 * b12; m[7] = a03 * b10 + a13 * b11 + a23 * b12;
        m[8] = a00 * b20 + a10 * b21 + a20 * b22; m[9] = a01 * b20 + a11 * b21 + a21 * b22;
        m[10] = a02 * b20 + a12 * b21 + a22 * b22; m[11] = a03 * b20 + a13 * b21 + a23 * b22;
    }
</script>

# h1 Heading 8-)

## h2 Heading

### h3 Heading

#### h4 Heading

##### h5 Heading

###### h6 Heading

***

Enable typographer option to see result.

"Smartypants, double quotes" and 'single quotes'

**This is bold text**

__This is bold text__

*This is italic text*

_This is italic text_

~~Strikethrough~~

> Blockquotes can also be nested...
> > ...by using additional greater-than signs right next to each other...
> > > ...or with spaces between arrows.

Unordered

- Create a list by starting a line with `+`, `-`, or `*`
- Sub-lists are made by indenting 2 spaces:
    - Marker character change forces new list start:
        - Ac tristique libero volutpat at

        - Facilisis in pretium nisl aliquet

        - Nulla volutpat aliquam velit
- Very easy!

Ordered

1. Lorem ipsum dolor sit amet
2. Consectetur adipiscing elit
3. Integer molestie lorem at massa

Start numbering with offset:

57. foo
1. bar

Inline `code`

Indented code

    // Some comments
    line 1 of code
    line 2 of code
    line 3 of code

Block code "fences"

```
Sample text here...
```

Syntax highlighting

```js
var foo = function (bar) {
    return bar++;
};

console.log(foo(5));
```

```cpp
void* allocate(size_t size) {
    return malloc(size);
}
```

| Option | Description                                                               |
|--------|---------------------------------------------------------------------------|
| data   | path to data files to supply the data that will be passed into templates. |
| engine | engine to be used for processing templates. Handlebars is the default.    |
| ext    | extension to be used for dest files.                                      |

[link text](http://dev.nodeca.com)

![Minion](https://octodex.github.com/images/minion.png)
*A GitHub minion*

Footnote 1 link[^first].

Footnote 2 link[^second].

Duplicated footnote reference[^second].

### Footnotes

[^first]: Footnote **can have markup**

    and multiple paragraphs.

[^second]: Footnote text.

> [!NOTE]
> Useful information that users should know, even when skimming content.

> [!TIP]
> Helpful advice for doing things better or more easily.

> [!IMPORTANT]
> Key information users need to know to achieve their goal.

> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.

> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.