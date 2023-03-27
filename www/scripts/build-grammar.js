const path = require('path')
const { promises: fs } = require('fs')

const grammkit = require('grammkit')
const { parse } = require('pegjs/lib/parser')

const main = async () => {
  const grammarDir = path.join(process.cwd(), 'content', '_static', 'grammar')

  const files = await fs.readdir(grammarDir)
  const grammarFiles = files.filter(filename => filename.endsWith('.peg'))

  const grammars = await Promise.all(
    grammarFiles.map(async filename => {
      const input = path.join(grammarDir, filename)
      console.log('Loading:', input)

      const buffer = await fs.readFile(input)
      const content = buffer.toString()

      const grammar = parse(content)

      let html = '<div class="grammar-diagram-container">'

      for (const rule of grammar.rules) {
        html += `
          <div class="rule">
            <p class="has-text-weight-bold">Rule: <code>${rule.name}</code></p>
            <div>${grammkit.diagram(rule)}</div>
          </div>
        `
      }

      html += `</div>`

      return {
        source: filename,
        target: `${path.parse(filename).name}.html`,
        content,
        html,
      }
    })
  )

  await Promise.all(
    grammars.map(async ({ target, html }) => {
      const out = path.join(grammarDir, target)
      console.log('Writing:', out)

      await fs.writeFile(out, html)
    })
  )
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
