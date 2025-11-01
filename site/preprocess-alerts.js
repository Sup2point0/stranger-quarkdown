/* Very scuffed workflow for implementing GitHub Alert blockquotes, but what can you do when MDsveX drops support for Unified ;-; */

import { visit } from "unist-util-visit";


export function remark_alerts() {
  return tree => {
    visit(tree, "blockquote", node => {
      let block = node.children?.[0];
      let line = block?.children?.[0];
      let content = line?.children?.[0];
      let kind = content?.value?.match(/!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)/i)?.[1];

      if (line?.type === "linkReference" && kind) {
        block.children.shift();  // remove old
        node.children.unshift({
          type: "paragraph",
          children: [{
            type: "text",
            value: kind.toUpperCase()
          }],
          data: {
            hProperties: {
              class: "gfm-alert-indicator"
            }
          },
        });
        
        node.data = {
          hName: "blockquote",
          hProperties: {
            class: `gfm-alert ${kind.toLowerCase()}`
          }
        }
      }
    });
  };
}
