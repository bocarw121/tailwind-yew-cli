# Tailwind Yew CLI

Easily integrate the latest version of Tailwind CSS into your existing Yew project.

## Installation 

Install the Tailwind Yew CLI by executing the following command:

```sh
cargo install tailwind-yew
```


## Usage

To get started, run the following command with your preferred options. The default options are as follows:

```sh
tailwind-yew add -p styles -i input.css -o output.css
```

You can also simply run:


```sh
tailwind-yew add
```

Once the command is executed, you'll be prompted to choose the executable for your platform.

The CLI will then take care of setting up everything necessary to integrate Tailwind CSS seamlessly into your Yew project. This includes building the Tailwind executable, creating the style directory, and adding the necessary HTML link. Your project will be ready to utilize Tailwind CSS. 

For ongoing development, run the generated executable with the watch flag to maintain the smooth development workflow:

```sh
./tailwindcss -i styles/input.css -o styles/output.css # or replaces with your input and output directory if you need not pick the defaults
```

This will enable you to observe real-time changes and ensure that your Yew project's styling remains consistent and up-to-date with the power of Tailwind CSS.

Take a look at the created `tailwind_info.md` for more information, as it dynamically generates the paths you previously selected. That's it, you're all set. Now go make something really cool 💪!
```sh
Options:
  -p, --path <PATH>                Optional - Path for the tailwind input and output css files [default: styles]
  -i, --input-name <INPUT_NAME>    Optional - Input css name You must add the extension ie. [my_input.css] - [default: input.css]
  -o, --output-name <OUTPUT_NAME>  Optional - Output css name You must add the extension ie. [my_output.css] - [default: output.css]
  -h, --help                       Print help information                    Print help information
```
