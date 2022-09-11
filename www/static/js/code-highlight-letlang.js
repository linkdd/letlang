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
        match: /^\b(module|import|as|pub|const|class|func|effect|perform|throw|intercept|catch|let|do|match|cond|else|loop|break|is|spawn|not|in|receive|after)\b/
      },
      {
        name: 'symbol',
        match: /^(\$|\=|\=\>|\:\=|\||\-\>|\<|\>|\.|\:|\&|\!|\<\<|\>\>|\/|\*|\+|\-|\|\>|\,|\;|\%|\{|\}|\(|\)|\[|\])/
      },
      {
        name: 'builtin-type',
        match: /^\b(boolean|int|number|string|atom|pid|list)\b/
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