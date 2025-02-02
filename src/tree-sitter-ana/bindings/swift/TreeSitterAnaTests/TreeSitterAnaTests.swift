import XCTest
import SwiftTreeSitter
import TreeSitterAna

final class TreeSitterAnaTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_ana())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Ana grammar")
    }
}
