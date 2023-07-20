[![codecov](https://codecov.io/gh/eZaplachi/ztcm/branch/main/graph/badge.svg?token=V8CJWT9BQK)](https://codecov.io/gh/eZaplachi/ztcm)

# Zap's Typed Css Modules

### A rust implementation of [Typed-Css-Modules](https://github.com/Quramy/typed-css-modules) cli.

Creates TypeScript definition files from [CSS Modules](https://github.com/css-modules/css-modules) .css files.

If you have the following css,

```css
/* styles.css */

@value primary: red;

.myClass {
  color: primary;
}
```

typed-css-modules creates the following .css.ts files from the above css:

```ts
/* styles.css.ts */
declare const styles: {
  readonly primary: string;
  readonly myClass: string;
};
export default styles;
```

So, you can import CSS modules' class or variable into your TypeScript sources:

```ts
/* app.ts */
import styles from "./styles.css";
console.log(`<div class="${styles.myClass}"></div>`);
console.log(`<div style="color: ${styles.primary}"></div>`);
```

## Install

> To be completed

## Commands

You can run the program with the following command:

```
ztcm [PATH('.' for cwd)] [FLAGS]
```

**Flags**

- `-h, --help` : Show help
- `-v, --version` : Show version number
- `-c, --camel_case`: Converts output from kebab-case to camelCase in .css.ts files (_Unstable_)
- `-r, --recursive`: Recursively search through the selected folder
- `-t, --timer`: Show timer;
- `-o , --output[=Output Directory]`: Chose which directory to output .css.ts files to (default: same dict as .module.css file)
- `-p, --pattern[=Pattern]`: Change search pattern from (default: '.module.css')
- `-w, --watch[=<DELAY(s)>]` Enable watch and optionally set the watch delay; calling the flag without a value defaults to (1.0s)
- `-u, --update-after-cycles[=<UPDATE_AFTER_CYCLES>]` Set number of _watch_ cycles to pass before re-indexing files (default: 45)

_This project was intended for learning purposes - not serious use_

### References

- https://github.com/BamPeers/rust-ci-github-actions-workflow
- https://users.rust-lang.org/t/how-to-remove-last-character-from-str/68607/2
