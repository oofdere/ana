package tree_sitter_ana_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_ana "github.com/oofdere/tree-sitter-ana/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_ana.Language())
	if language == nil {
		t.Errorf("Error loading Ana grammar")
	}
}
