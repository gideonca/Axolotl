#version 330 core

layout (location = 0) in vec3 aPosition; // Input variable for the vertex position

layout (location = 1) in vec3 aColor; // Input variable for the vertex color

//out vec3 vertexColor; // Output variable for the vertex color
out vec3 ourColor; // Output variable for the vertex color

void main()
{
    gl_Position = vec4(aPosition, 1.0); // Set the position of the current vertex
    ourColor = aColor; // Set the color of the current vertex
    //vertexColor = vec4(0.5, 0.0, 0.0, 1.0); // Set the vertex color to red
}