# Squarkdown Docs
<!-- #SQUARK live!
| dest = docs
-->

Welcome to the *Squarkdown* docs, where you can find all you need to know about Squarkdown!

```js
/**
 * A $state() rune persisted to `localStorage`.
 */
export class PersistedState<T>
{
  #key: string;
  value = $state<T>() as T;

  get key() {
    return this.#key;
  }

  constructor(key: string, init: T)
  {
    this.#key = key ?? "";
    this.value = init;

    if (browser) {
      let data = localStorage.getItem(this.key);
      if (data) {
        this.value = JSON.parse(data);
      }
    }

    $effect.root(() => {
      $effect(() => {
        localStorage?.setItem(this.key, JSON.stringify(this.value));
      })
    });
  }
}
```
