# Configuring Squarkup for a Repository
<!-- #SQUARK live!
| dest = docs/walkthrough/repo-config
-->

To use Squarkdown in a repo, you’ll need 2 things:

- The [StrangerQuarkdown repo](https://github.com/Sup2point0/stranger-quarkdown) included in your project as a Git submodule
- A `squarkup.json` file inside a `.squarkdown/` folder at the root of your project

This means your project files will include these directories:

```
./
   .squarkdown/
      squarkup.json
   stranger-quarkdown/
      ...
```

> [!Note]
> Since you are free to name the `stranger-quarkdown` submodule directory whatever you wish, we will refer to it as the “StrangerQuarkdown directory” throughout the rest of this page to avoid ambiguity.


<br>


## StrangerQuarkdown Submodule

The Git submodule stores Squarkdown’s code. When you want to run Squarkdown, you’ll `cd` into this directory and run `rake squark`.

### Adding the Submodule
In your project root, run the following command:

```bash
git submodule add https://github.com/Sup2point0/stranger-quarkdown
```

This clones the StrangerQuarkdown repo into the `stranger-quarkdown/` folder. That folder is a Git submodule, so it’ll look a little different in most IDEs and on GitHub.

If you’d like to give the folder a different name, just add your alias at the end of the command. I recommend `.stranger-quarkdown` for separating it from your actual project files:

```bash
git submodule add https://github.com/Sup2point0/stranger-quarkdown .stranger-quarkdown
```

### Updating the Submodule
As Squarkdown receives updates and fixes, you’ll want to pull them into your project. To do this, just run the following command:

```bash
git submodule update
```

> [!Tip]
> If you have other Git submodules in your project, you may want to run this command in your StrangerQuarkdown directory to avoid unintentionally updating those other Git submodules.

### Cloning the Submodule
Bear in mind that submodules are not cloned by default when cloning their parent repository. If you’re cloning your project that uses Squarkdown, make sure to include the `--recurse-submodules` flag:

```bash
git clone <project-url> --recurse-submodules
```

If you’ve already cloned your project repo and the StrangerQuarkdown directory is empty, you can still clone it by pulling with the `--init` flag:

```bash
git submodule update --init
```


<br>


## `.squarkdown/`

This directory contains your project’s `squarkup.json` for configuring squarkup. You can also place any additional Markdown files you want to squarkup here.

You can either create your `squarkup.json` manually, or let the new Squarkdown CLI guide you through it.

### Creating `squarkup.json` Manually
For better IDE support, make sure to start your JSON body with a reference to a `squarkup.json` schema:

```json
{
  "$schema": ""
}
```

You can find a list of available schemas at [Squarkup Schemas](https://sup2point0.github.io/stranger-quarkdown/squarkup-schemas).

The body has no nesting and contains keys with the format `<category> / <field>`. For the available and required fields, refer to [reference/squarkup.json](../reference/squarkup.json).

### Using the Squarkdown CLI

> *New in v3.0*.

In your StrangerQuarkdown directory, run this:

```bash
rake init
```

All you need to do is follow the CLI, which will provide you with sensible options and automatically write your `squarkup.json` for you!
