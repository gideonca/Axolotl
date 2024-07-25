#version 330 core

out vec4 FragColor;
in vec3 ourColor;

//in vec4 vertexColor; // the input variable from the vertex shader (same name and same type)

//uniform vec4 ourColor; // we set this variable in the OpenGL code

void main()
{
    //FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f); // set the output variable to orange

    //FragColor = vertexColor;      // passed in from vertex shader, dark red

    //FragColor = ourColor; // set in OpenGL code, go from green to black and back to green
    
    FragColor = vec4(ourColor, 1.0);
}