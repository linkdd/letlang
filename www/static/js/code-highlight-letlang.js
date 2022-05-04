$(() => {
  const selector = 'pre code.language-letlang'

  $(selector).each((idx, codeblock) => {
    $(codeblock).parent().css({
      color: '#f8f8f2',
      'background-color': '#272822',
      '-moz-tab-size': '4',
      '-o-tab-size': '4',
      'tab-size': '4'
    })
  })

  window.csHighlight({
    patterns: [
      {
        name: 'mainfunc',
        match: /^\b(main)\b/
      },
      {
        name: 'atom-literal',
        match: /^(\@\w+|true|false)/
      },
      {
        name: 'number-literal',
        match: /^(\d+(\.\d+)?)/
      },
      {
        name: 'keyword',
        match: /^\b(module|import|as|export|class|solvable|thereis|forall|func|effect|perform|throw|catch|finally|let|do|match|if|else|is|coro|join|not|in)\b/
      },
      {
        name: 'symbol',
        match: /^(\=|\=\>|\:\=|\||\-\>|\<|\>|\.|\:|\&|\!|\<\<|\>\>|\/|\*|\+|\-|\|\>|\,|\;)/
      },
      {
        name: 'builtin-type',
        match: /^\b(boolean|number|string|atom|stream|set|list)\b/
      },
      {
        name: 'string-literal',
        match: /^(\"[^\"\n]*\")/
      },
      {
        name: 'comment',
        match: /^(\#[^\n]*)/
      }
    ],
    selector
  })
})