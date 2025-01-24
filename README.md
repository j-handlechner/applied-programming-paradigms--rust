__Assignment Game of Life - Johannes Handlechner__  
Deployed site: [https://marvelous-nougat-ef9868.netlify.app/](https://marvelous-nougat-ef9868.netlify.app/)  
Deployed site (interactive.html): [https://marvelous-nougat-ef9868.netlify.app/interactive](https://marvelous-nougat-ef9868.netlify.app/interactive)

# Important info
I built a little extention to the assignment:
- page index.html calls the .life() function as you specified in the assignment description (with one parameter being the iteration/index). All of the context switches thus get handled in JS. Because I'm passing the iteration that the function should compute to, I had the feeling that a lot of unnecessary opterations were done (for iteration 10 i need to callculate all the steps before and then render the last one, for iteration 11 i need to do all of this again + 1 step) -> this is because i wanted to do another additional solution (with a slightly different function notation for the call in interactive.html)
  
- page interactive.html calls the function .append_life_iteration(). Why? I wanted to create another version that handles the timeout in rust -> no context switches for the setInterval and that only computes each step once and "appends" it to the last one. And it just keeps going and doesn't stop after 15 Iterations. It additionally scales to be somewhat filling your screen (no fixed size) - hence the name.