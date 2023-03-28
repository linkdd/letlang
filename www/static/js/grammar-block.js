$(() => {
  $('.grammar-block').each((_index, block) => {
    const root = $(block)

    const nav = root.find('.grammar-block-nav')
    const content = root.find('.grammar-block-content')

    const activateTab = tabId => {
      nav.find(`li[data-tabid="${tabId}"]`).addClass('is-active')
      content.find(`div[data-tabid="${tabId}"]`).addClass('is-active')
    }

    const deactivateTab = tabId => {
      nav.find(`li[data-tabid="${tabId}"]`).removeClass('is-active')
      content.find(`div[data-tabid="${tabId}"]`).removeClass('is-active')
    }

    activateTab(root.data('tabid'))

    nav.find('li a').each((_index, tabLink) => {
      const el = $(tabLink)
      const tabId = el.parent().data('tabid')

      el.click(() => {
        deactivateTab(root.data('tabid'))
        root.data('tabid', tabId)
        activateTab(root.data('tabid'))
      })
    })
  })
})
