window.BaseWebComponent = {
  extends({ tag, template, render }) {
    customElements.define(tag, class extends HTMLElement {
      connectedCallback() {
        const fragment = document.getElementById(template)
        const node = document.importNode(fragment.content, true)

        $(this).append(node)
        render($(this))
      }
    })
  }
}
