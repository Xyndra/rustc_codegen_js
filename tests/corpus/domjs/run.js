// Node driver: fake `document` for the domjs corpus, load the `dom`
// support bundle (real functions like `document()` live there) via a path
// relative to argv[2], then call `run()` and print.
import { pathToFileURL } from "node:url";
function makeEl(id, extra) {
    const el = {
        tagName: "DIV",
        html: "",
        parent: null,
        replaceChildren() {},
        insertAdjacentHTML(_pos, html) {
            this.html += html;
        },
        getAttribute(name) {
            return name === "id" ? id : null;
        },
        closest(selectors) {
            if (selectors === "!!!invalid") {
                throw new SyntaxError("not a selector");
            }
            if (selectors === ".nomatch") {
                return null;
            }
            return this.parent;
        },
        ...extra,
    };
    return el;
}
const els = {
    stage: makeEl("stage"),
    cmd: makeEl("cmd", { tagName: "TEXTAREA", value: "push" }),
};
els.stage.parent = makeEl("parent-div");
const document = {
    getElementById(id) {
        return els[id] ?? null;
    },
    querySelector(selectors) {
        if (selectors === "#stage") {
            return els.stage;
        }
        return null;
    },
};
globalThis.document = document;
globalThis.window = { document };
const corpusUrl = pathToFileURL(process.argv[2]);
await import(new URL("../support/dom.mjs", corpusUrl).href);
const m = await import(corpusUrl.href);
console.log(m.run());
