// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    tests::testutils_composer_model::{cm, restore_whitespace, tx},
    ComposerModel, TextUpdate,
};

#[test]
fn backspacing_a_character_at_the_end_deletes_it() {
    let mut model = cm("abc|");
    model.backspace();
    assert_eq!(tx(&model), "ab|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn backspacing_a_character_at_the_beginning_does_nothing() {
    let mut model = cm("|abc");
    model.backspace();
    assert_eq!(tx(&model), "|abc");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn backspacing_a_character_in_the_middle_deletes_it() {
    let mut model = cm("ab|c");
    model.backspace();
    assert_eq!(tx(&model), "a|c");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn backspacing_a_selection_deletes_it() {
    let mut model = cm("a{bc}|");
    model.backspace();
    assert_eq!(tx(&model), "a|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn backspacing_a_backwards_selection_deletes_it() {
    let mut model = cm("a|{bc}");
    model.backspace();
    assert_eq!(tx(&model), "a|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
#[allow(deprecated)]
fn backspacing_a_lone_line_break_deletes_it() {
    let mut model = ComposerModel::new();
    model.add_line_break();
    model.backspace();
    assert_eq!(tx(&model), "|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
#[allow(deprecated)]
fn backspacing_a_line_break_deletes_it() {
    let mut model = cm("abc|");
    let update = model.add_line_break();

    let replace_all = match update.text_update {
        TextUpdate::Keep => panic!("expected ReplaceAll"),
        TextUpdate::ReplaceAll(replace_all) => replace_all,
        TextUpdate::Select(_) => panic!("expected ReplaceAll"),
    };

    assert_eq!(replace_all.start, 4);
    assert_eq!(replace_all.end, 4);

    model.backspace();
    model.backspace();
    assert_eq!(tx(&model), "ab|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_character_at_the_end_does_nothing() {
    let mut model = cm("abc|");
    model.delete();
    assert_eq!(tx(&model), "abc|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_character_at_the_beginning_deletes_it() {
    let mut model = cm("|abc");
    model.delete();
    assert_eq!(tx(&model), "|bc");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_character_in_the_middle_deletes_it() {
    let mut model = cm("a|bc");
    model.delete();
    assert_eq!(tx(&model), "a|c");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_selection_deletes_it() {
    let mut model = cm("a{bc}|");
    model.delete();
    assert_eq!(tx(&model), "a|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_backwards_selection_deletes_it() {
    let mut model = cm("a|{bc}");
    model.delete();
    assert_eq!(tx(&model), "a|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_range_removes_it() {
    let mut model = cm("abcd|");
    model.delete_in(1, 3);
    assert_eq!(tx(&model), "a|d");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_when_spanning_two_separate_identical_tags_joins_them() {
    let mut model = cm("<b>bo{ld</b> plain <b>BO}|LD</b>");
    model.delete();
    assert_eq!(tx(&model), "<b>bo|LD</b>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_across_list_items_joins_them() {
    let mut model = cm("<ol>\
            <li>1{1</li>\
            <li>22</li>\
            <li>33</li>\
            <li>4}|4</li>\
        </ol>");
    model.delete();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<ol>\
            <li>1|4</li>\
        </ol>"
    );
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_across_lists_joins_them() {
    let mut model = cm("<ol>\
            <li>1{1</li>\
            <li>22</li>\
        </ol>\
        <ol>\
            <li>33</li>\
            <li>4}|4</li>\
        </ol>");
    model.delete();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>1|4</li></ol>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_across_lists_joins_them_nested() {
    let mut model = cm("<ol>\
            <li>1{1</li>\
            <li>22</li>\
            <ol>\
                <li>55</li>\
            </ol>\
        </ol>\
        <ol>\
            <li>33</li>\
            <li>4}|4</li>\
        </ol>");
    model.delete();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>1|4</li></ol>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_across_formatting_different_types() {
    let mut model = cm("<b><i>some {italic</i></b> and}| <b>bold</b> text");
    model.delete();
    assert_eq!(tx(&model), "<b><i>some&nbsp;|</i></b> <b>bold</b> text");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_across_formatting_different_types_on_node_boundary() {
    let mut model = cm("<b><i>some {italic</i></b> and }|<b>bold</b> text");
    model.delete();
    assert_eq!(tx(&model), "<b><i>some&nbsp;|</i>bold</b> text");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_in_nested_structure_and_format_nodes_works() {
    let mut model = cm("<ul><li>A</li><li><b>B{B</b><b>C}|C</b></li></ul>");
    model.delete();
    assert_eq!(tx(&model), "<ul><li>A</li><li><b>B|C</b></li></ul>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_empty_list_item() {
    let mut model = cm("<ul><li>A{</li><li>}|</li></ul>");
    model.backspace();
    assert_eq!(tx(&model), "<ul><li>A|</li></ul>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_a_newline_deletes_it() {
    let mut model = cm("abc|<br />def");
    model.delete();
    model.delete();
    assert_eq!(tx(&model), "abc|ef");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn test_backspace_emoji() {
    let mut model = cm("😄|😅");
    model.backspace();
    assert_eq!(tx(&model), "|😅");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn test_backspace_complex_emoji() {
    let mut model = cm("Test😮‍💨|😅");
    model.backspace();
    assert_eq!(tx(&model), "Test|😅");
    model.select(6.into(), 6.into());
    model.backspace();
    assert_eq!(tx(&model), "Test|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn test_delete_emoji() {
    let mut model = cm("😄|😅");
    model.delete();
    assert_eq!(tx(&model), "😄|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn test_delete_complex_emoji() {
    let mut model = cm("Test😮‍💨|😅");
    model.delete();
    assert_eq!(tx(&model), "Test😮‍💨|");
    model.select(4.into(), 4.into());
    model.delete();
    assert_eq!(tx(&model), "Test|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn test_delete_complex_grapheme() {
    let mut model = cm("Test|О́");
    model.delete();
    assert_eq!(tx(&model), "Test|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn test_backspace_complex_grapheme() {
    let mut model = cm("TestО́|");
    model.backspace();
    assert_eq!(tx(&model), "Test|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_initial_text_node_removes_it_completely_without_crashing() {
    let mut model = cm("abc<br />def<br />gh|");
    model.delete_in(4, 10);
    assert_eq!(tx(&model), "abc<br />|",);
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_initial_text_node_via_selection_removes_it_completely() {
    let mut model = cm("abc<br />{def<br />gh}|");
    model.delete();
    assert_eq!(tx(&model), "abc<br />|",);
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_all_initial_text_and_merging_later_text_produces_one_text_node() {
    let mut model = cm("abc<br />{def<br />gh}|ijk");
    model.delete();
    assert_eq!(tx(&model), "abc<br />|ijk",);
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_all_initial_text_within_a_tag_preserves_the_tag() {
    let mut model = cm("abc<br /><strong>{def<br />gh}|ijk</strong>");
    model.delete();
    assert_eq!(tx(&model), "abc<br />|<strong>ijk</strong>",);
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_all_text_within_a_tag_deletes_the_tag() {
    let mut model = cm("abc<br /><strong>{def<br />gh}|</strong>ijk");
    model.delete();
    assert_eq!(tx(&model), "abc<br />|ijk",);
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_last_character_in_a_container() {
    let mut model = cm("<b>t|</b>");
    model.backspace();
    assert_eq!(tx(&model), "|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_selection_in_a_container() {
    let mut model = cm("<b>{test}|</b>");
    model.backspace();
    assert_eq!(tx(&model), "|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_selection_in_multiple_containers() {
    let mut model = cm("<i><b>{test}|</b></i>");
    model.backspace();
    assert_eq!(tx(&model), "|");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_selection_of_a_container_in_multiple_containers() {
    let mut model = cm("<i><b>{test}|</b> test</i>");
    model.backspace();
    assert_eq!(tx(&model), "<i>|&nbsp;test</i>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_selection_of_a_container_with_text_node_neighbors() {
    let mut model = cm("<em>abc<del>{def}|</del>ghi</em>");
    model.backspace();
    assert_eq!(tx(&model), "<em>abc|ghi</em>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn deleting_selection_of_a_container_with_matching_neighbors() {
    let mut model = cm(
        "<em><strong>abc</strong><del>{def}|</del><strong>ghi</strong></em>",
    );
    model.backspace();
    assert_eq!(tx(&model), "<em><strong>abc|ghi</strong></em>");
    model.state.dom.explicitly_assert_invariants();
}

// Remove word tests, text only. nb these _may_ be considered as superseded by the
// html tests which repeat these exact tests, but wrapped in an <em> tag
#[test]
fn plain_backspace_word_at_beginning_does_nothing() {
    let mut model = cm("|abc");
    model.backspace_word();
    assert_eq!(tx(&model), "|abc")
}
#[test]
fn plain_delete_word_at_end_does_nothing() {
    let mut model = cm("abc|");
    model.delete_word();
    assert_eq!(tx(&model), "abc|")
}

#[test]
fn plain_backspace_word_with_selection_only_removes_selection() {
    let mut model = cm("ab{c def}|");
    model.backspace_word();
    assert_eq!(tx(&model), "ab|")
}
#[test]
fn plain_delete_word_with_selection_only_removes_selection() {
    let mut model = cm("ab{c def}|");
    model.delete_word();
    assert_eq!(tx(&model), "ab|")
}

#[test]
fn plain_backspace_word_at_end_of_single_word_removes_word() {
    let mut model = cm("abc|");
    model.backspace_word();
    assert_eq!(tx(&model), "|")
}
#[test]
fn plain_delete_word_at_start_of_single_word_removes_word() {
    let mut model = cm("|abc");
    model.delete_word();
    assert_eq!(tx(&model), "|")
}

#[test]
fn plain_backspace_word_in_word_removes_start_of_word() {
    let mut model = cm("ab|c");
    model.backspace_word();
    assert_eq!(tx(&model), "|c")
}
#[test]
fn plain_delete_word_in_word_removes_end_of_word() {
    let mut model = cm("a|bc");
    model.delete_word();
    assert_eq!(tx(&model), "a|")
}

#[test]
fn plain_backspace_word_with_multiple_words_removes_single_word() {
    let mut model = cm("abc def| ghi");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "abc | ghi")
}
#[test]
fn plain_delete_word_with_multiple_words_removes_single_word() {
    let mut model = cm("abc |def ghi");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "abc | ghi")
}

#[test]
fn plain_backspace_word_removes_whitespace_then_word() {
    let mut model = cm("abc def          |");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "abc |")
}
#[test]
fn plain_delete_word_removes_whitespace_then_word() {
    let mut model = cm("|          abc def");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "| def")
}

#[test]
fn plain_backspace_word_removes_runs_of_non_word_characters() {
    let mut model = cm("abc,.()!@$^*|");
    model.backspace_word();
    assert_eq!(tx(&model), "abc|")
}
#[test]
fn plain_delete_word_removes_runs_of_non_word_characters() {
    let mut model = cm("|,.()!@$^*abc");
    model.delete_word();
    assert_eq!(tx(&model), "|abc")
}

#[test]
fn plain_backspace_word_removes_runs_of_non_word_characters_and_whitespace() {
    let mut model = cm("abc  ,.!@$%       |");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "abc  |")
}
#[test]
fn plain_delete_word_removes_runs_of_non_word_characters_and_whitespace() {
    let mut model = cm("|  ,.!@$%  abc");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|  abc")
}

// Remove word tests including html
#[test]
fn html_backspace_word_at_beginning_does_nothing() {
    let mut model = cm("<em>|abc</em>");
    model.backspace_word();
    assert_eq!(tx(&model), "<em>|abc</em>")
}
#[test]
fn html_delete_word_at_end_does_nothing() {
    let mut model = cm("<em>abc|</em>");
    model.delete_word();
    assert_eq!(tx(&model), "<em>abc|</em>")
}

#[test]
fn html_backspace_word_with_selection_only_removes_selection() {
    let mut model = cm("<em>ab{c def}|</em>");
    model.backspace_word();
    assert_eq!(tx(&model), "<em>ab|</em>")
}
#[test]
fn html_delete_word_with_selection_only_removes_selection() {
    let mut model = cm("<em>ab{c def}|</em>");
    model.delete_word();
    assert_eq!(tx(&model), "<em>ab|</em>")
}

#[test]
fn html_backspace_word_at_end_of_single_word_removes_word() {
    let mut model = cm("<em>abc|</em>");
    model.backspace_word();
    assert_eq!(tx(&model), "|")
}
#[test]
fn html_delete_word_at_start_of_single_word_removes_word() {
    let mut model = cm("<em>|abc</em>");
    model.delete_word();
    assert_eq!(tx(&model), "|")
}

#[test]
fn html_backspace_word_in_word_removes_start_of_word() {
    let mut model = cm("<em>ab|c</em>");
    model.backspace_word();
    assert_eq!(tx(&model), "<em>|c</em>")
}
#[test]
fn html_delete_word_in_word_removes_end_of_word() {
    let mut model = cm("<em>a|bc</em>");
    model.delete_word();
    assert_eq!(tx(&model), "<em>a|</em>")
}

#[test]
fn html_backspace_word_with_multiple_words_removes_single_word() {
    let mut model = cm("<em>abc def| ghi</em>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>abc | ghi</em>")
}
#[test]
fn html_delete_word_with_multiple_words_removes_single_word() {
    let mut model = cm("<em>abc |def ghi</em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>abc | ghi</em>")
}

#[test]
fn html_backspace_word_removes_whitespace_then_word() {
    let mut model = cm("<em>abc def          |</em>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>abc |</em>")
}
#[test]
fn html_delete_word_removes_whitespace_then_word() {
    let mut model = cm("<em>|          abc def</em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>| def</em>")
}

#[test]
fn html_backspace_word_removes_runs_of_non_word_characters() {
    let mut model = cm("<em>abc,.()!@$^*|</em>");
    model.backspace_word();
    assert_eq!(tx(&model), "<em>abc|</em>")
}
#[test]
fn html_delete_word_removes_runs_of_non_word_characters() {
    let mut model = cm("<em>|,.()!@$^*abc</em>");
    model.delete_word();
    assert_eq!(tx(&model), "<em>|abc</em>")
}

#[test]
fn html_backspace_word_removes_runs_of_non_word_characters_and_whitespace() {
    let mut model = cm("<em>abc  ,.!@$%       |</em>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>abc  |</em>")
}
#[test]
fn html_delete_word_removes_runs_of_non_word_characters_and_whitespace() {
    let mut model = cm("<em>|  ,.!@$%  abc</em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>|  abc</em>")
}

#[test]
fn html_backspace_word_removes_single_linebreak() {
    let mut model = cm("<br />|");
    model.backspace_word();
    assert_eq!(tx(&model), "|")
}
#[test]
fn html_delete_word_removes_single_linebreak() {
    let mut model = cm("|<br />");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|")
}

#[test]
fn html_backspace_word_removes_only_one_linebreak_of_many() {
    let mut model = cm("<br /><br />|<br />");
    model.backspace_word();
    assert_eq!(tx(&model), "<br />|<br />");
    model.backspace_word();
    assert_eq!(tx(&model), "|<br />")
}
#[test]
fn html_delete_word_removes_only_one_linebreak_of_many() {
    let mut model = cm("<br />|<br /><br />");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<br />|<br />");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<br />|")
}

#[test]
fn html_backspace_word_does_not_remove_past_linebreak_in_word() {
    let mut model = cm("a<br />defg|");
    model.backspace_word();
    assert_eq!(tx(&model), "a<br />|")
}
#[test]
fn html_delete_word_does_not_remove_past_linebreak_in_word() {
    let mut model = cm("|abcd<br />f ");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|<br />f ")
}

#[test]
fn html_backspace_word_at_linebreak_removes_linebreak() {
    let mut model = cm("abc <br/>|");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "abc |");
}
#[test]
fn html_delete_word_at_linebreak_removes_linebreak() {
    let mut model = cm("|<br/> abc");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "| abc");
}

#[test]
fn html_backspace_word_removes_past_linebreak_in_whitespace() {
    let mut model = cm("abc <br/> |");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "abc |");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|");
}
#[test]
fn html_delete_word_removes_past_linebreak_in_whitespace() {
    let mut model = cm("| <br/> abc");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "| abc");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|");
}

#[test]
fn html_backspace_word_removes_whole_word() {
    let mut model = cm("<em>italic|</em>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|");
}
#[test]
fn html_delete_word_removes_whole_word() {
    let mut model = cm("<em>|italic</em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|");
}

#[test]
fn html_backspace_word_removes_into_a_tag() {
    let mut model = cm("<em>some em</em>phasis|");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>some |</em>");
}
#[test]
fn html_delete_word_removes_into_a_tag() {
    let mut model = cm("|so<em>me emphasis</em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>| emphasis</em>");
}

#[test]
fn html_backspace_word_removes_through_a_tag() {
    let mut model = cm("si<em>ng</em>le|");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|");
}
#[test]
fn html_delete_word_removes_through_a_tag() {
    let mut model = cm("|si<em>ng</em>le");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "|");
}

#[test]
fn html_backspace_word_removes_between_tags() {
    let mut model = cm("<em>start spl</em><strong>it</strong>| end");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>start |</em> end");
}
#[test]
fn html_delete_word_removes_between_tags() {
    let mut model = cm("<em>start |spl</em><strong>it</strong> end");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>start |</em> end");
}

#[test]
fn html_backspace_word_removes_between_nested_tags() {
    let mut model = cm("<em><em>start spl</em></em><strong>it</strong>| end");
    model.backspace_word();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<em><em>start |</em></em> end"
    );
}
#[test]
fn html_delete_word_removes_between_nested_tags() {
    let mut model = cm("<em><em>start |spl</em></em><strong>it</strong> end");
    model.delete_word();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<em><em>start |</em></em> end"
    );
}

#[test]
fn html_backspace_word_into_deep_nesting() {
    let mut model = cm("<em>remains <em>all<em>of<em>the<em>rest</em>goes</em>away</em>x</em>y|</em>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>remains |</em>");
    model.state.dom.explicitly_assert_invariants();
}
#[test]
fn html_delete_word_into_deep_nesting() {
    let mut model = cm("<em>remains |<em>all<em>of<em>the<em>rest</em>goes</em>away</em>x</em>y</em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em>remains |</em>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn html_backspace_word_out_of_deep_nesting() {
    let mut model =
        cm("<em><em>stop <em><em><em>removethis|</em></em></em></em></em>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em><em>stop |</em></em>");
    model.state.dom.explicitly_assert_invariants();
}
#[test]
fn html_delete_word_out_of_deep_nesting() {
    let mut model =
        cm("<em><em><em><em><em>|removethis</em></em></em> stop</em></em>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<em><em>| stop</em></em>");
    model.state.dom.explicitly_assert_invariants();
}

#[test]
fn html_backspace_word_inside_single_list_item() {
    let mut model =
        cm("<ol><li>remove\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}|</li></ol>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>|</li></ol>");
}
#[test]
fn html_delete_word_inside_single_list_item() {
    let mut model = cm("<ol><li>|    remove</li></ol>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>|</li></ol>");
}

#[test]
fn html_backspace_word_does_not_move_outside_list_item() {
    let mut model = cm("<ol><li>1</li><li>12|</li><li>123</li></ol>");
    model.backspace_word();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<ol><li>1</li><li>|</li><li>123</li></ol>"
    );
}
#[test]
fn html_delete_word_does_not_move_outside_list_item() {
    let mut model = cm("<ol><li>1</li><li>|12</li><li>123</li></ol>");
    model.delete_word();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<ol><li>1</li><li>|</li><li>123</li></ol>"
    );
}

#[test]
fn backspace_between_block_nodes() {
    let mut model = cm("<p>First</p><p>|Second</p>");
    model.backspace();
    assert_eq!(tx(&model), "<p>First|Second</p>");
}

#[test]
fn backspace_between_nested_block_nodes() {
    let mut model = cm("<p>First</p><blockquote><p>|Second</p></blockquote>");
    model.backspace();
    assert_eq!(tx(&model), "<p>First|Second</p>");
}

#[test]
// TODO: remove these tests when implementing list behaviour
fn html_backspace_word_does_not_change_model() {
    let mut model = cm("<ol><li>|</li></ol>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>|</li></ol>");
}
#[test]
// TODO: remove these tests when implementing list behaviour
fn html_delete_word_does_not_change_model() {
    let mut model = cm("<ol><li>|</li></ol>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>|</li></ol>");
}

#[test]
fn html_backspace_word_for_single_empty_list_item() {
    let mut model = cm("<ol><li>|</li></ol>");
    model.backspace_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>|</li></ol>");
}
#[test]
fn html_delete_word_for_single_empty_list_item() {
    let mut model = cm("<ol><li>|</li></ol>");
    model.delete_word();
    assert_eq!(restore_whitespace(&tx(&model)), "<ol><li>|</li></ol>");
}

#[test]
fn html_backspace_word_for_empty_list_item() {
    let mut model = cm("<ol><li>1</li><li>|</li><li>123</li></ol>");
    model.backspace_word();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<ol><li>1</li><li>|</li><li>123</li></ol>"
    );
}
#[test]
fn html_delete_word_for_empty_list_item() {
    let mut model = cm("<ol><li>1</li><li>|</li><li>123</li></ol>");
    model.delete_word();
    assert_eq!(
        restore_whitespace(&tx(&model)),
        "<ol><li>1</li><li>|</li><li>123</li></ol>"
    );
}
