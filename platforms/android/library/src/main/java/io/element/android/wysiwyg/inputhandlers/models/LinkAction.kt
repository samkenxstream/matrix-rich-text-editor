package io.element.android.wysiwyg.inputhandlers.models

/**
 * Link related editor actions, depending on the current selection.
 */
sealed class LinkAction {
    /**
     * Insert new text with a link (only available when no text is selected)
     */
    object InsertLink : LinkAction()

    /**
     * Add or change the link for the current selection, without supplying text.
     */
    data class SetLink(val currentLink: String?) : LinkAction()
}
