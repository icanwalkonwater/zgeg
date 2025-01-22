package tree_sitter_zgeg_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_zgeg "github.com/icanwalkonwater/zgeg.git/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_zgeg.Language())
	if language == nil {
		t.Errorf("Error loading Zgeg grammar")
	}
}
