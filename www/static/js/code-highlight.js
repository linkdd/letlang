$(() => {
  const main = async () => {
    const languages = ['letlang', 'pegjs']
    const theme = 'monokai'

    const languageDefs = await Promise.all(languages.map(async lang => {
      const resp = await fetch(`/syntax/${lang}.tmLanguage.json`)
      const grammar = await resp.json()

      return {
        id: lang,
        scopeName: `source.${lang}`,
        grammar,
        aliases: [lang]
      }
    }))

    const highlighter = await shiki.getHighlighter({
      theme,
      langs: [],
    })
    await Promise.all(languageDefs.map(highlighter.loadLanguage))

    const highlightCode = lang => {
      let styles = { pre: '', code: '' }

      $(`pre code.language-${lang}`).each((_index, codeblock) => {
        const code = $('<div/>').html(codeblock.innerHTML).text();

        const tokens = highlighter.codeToThemedTokens(code, lang)
        codeblock.innerHTML = shiki.renderToHtml(tokens, {
          fg: highlighter.getForegroundColor(theme),
          bg: highlighter.getBackgroundColor(theme),
          elements: {
            pre({ style, children }) {
              $(codeblock).parent().attr('style', style)
              return `${children}`
            },
            code({ style, children }) {
              $(codeblock).attr('style', style)
              return `${children}`
            }
          }
        })
      })
    }

    languages.map(highlightCode)
  }

  main().catch(console.error)
})
