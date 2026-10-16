// Node driver: fake `document` for the dom corpus, then call `run()`.
const els = {};
globalThis.document = {
    getElementById(id) {
        if (!els[id]) {
            els[id] = {
                tagName: "DIV",
                html: "",
                replaceChildren() {},
                insertAdjacentHTML(_pos, html) {
                    this.html += html;
                },
                getAttribute(name) {
                    return name === "id" ? id : "";
                },
            };
        }
        return els[id];
    },
};
const m = await import(process.argv[2]);
console.log(m.run());
