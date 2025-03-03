# Setup

> [!WARNING]
> This guide assumes that you have scaffolded a hApp with the [TNESH stack template](https://darksoil.studio/tnesh-stack).

1. Run this to scaffold this zome in your hApp:

```bash
nix run github:darksoil-studio/contacts-zome#scaffold
```

This will do the following:
  - Add the `github:darksoil-studio/contacts-zome` flake input in your `flake.nix`.
  - Add the `contacts` coordinator and integrity zome packages to the `dna.nix` that you select.
  - Add the UI package for `@darksoil-studio/contacts-zome` as a dependency of your UI package.
  - Add the `<contacts-context>` element at the top level of your application.

That's it! You have now integrated the `linked_devices` coordinator and integrity zomes and their UI into your app!

Now, [choose which elements you need](/elements/contacts-context.md) and import them like this:

```js
import "@darksoil-studio/contacts-zome/dist/elements/contacts-context.js";
```

And then they are ready be used just like any other HTML tag. 

> [!NOTE]
> Importing the elements from the UI package will define them in the global `CustomElementsRegistry`, which makes them available to be used like any normal HTML tag. You can read more about custom elements [here](https://darksoil.studio/tnesh-stack/guides/custom-elements).

# Example

You can see a full working example of the UI working in [here](https://github.com/darksoil-studio/contacts-zome/blob/main/ui/demo/index.html).

