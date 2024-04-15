# Devlog 1

## Goals
- [ ] All rays are collectively represented using numpy arrays.
- [ ] Sphere intersection handler class
- [ ] Render a solid sphere
- [x] Export results to image

## Script
TODO... 
(Explain visuals + demos with, well, visuals) 
(explain code with codesnap images) 
(When one todo item is complete, show codesnap of me crossing it out)

Hey there, welcome to my first devlog. Today, we're going to be starting with a new project.
We're going to make a realtime ray tracing engine in python, and our main goal is that it should run smoothly even without GPU acceleration.
What's special about this series is that I know almost nothing about the technical aspects of ray tracing, so we will be learning this together.
Feel free to follow along. All of the code is open-source and available on GitHub. Link in the description.
With that aside, let's get started.

Unless you've been living under a rock, you probably know what ray tracing is. 
Raytracing is the next frontier in computer graphics, and we are getting closer and closer to perfecting this art.
As opposed to Rasterization, which maps the positions of triangles on a flat screen, raytracing simulates light rays as they would interact with objects in real life.
Now, I won't go into the details here, there are a ton of good videos that explain ray tracing better than I ever can.

Now, I know what you're thinking. A ray tracing engine, in PYTHON? Yes, you heard it right.
We will be applying some optimizations. And by optimizations, I mean...
Anyways, we will start with an image renderer. When it gets fast enough, we will move on to realtime scenes.
Let's get on with the code!

As I said earlier, we need some kind of image renderer.
Our renderer will output the final image as a 3d numpy array, and we need to save this to an image file. 
For that, we'll use pillow.
Let's create an export image function, which takes the 3d numpy array and the image filename.
This function first converts the array into an pillow image, and then saves the image.
That's it for the image exporter!
