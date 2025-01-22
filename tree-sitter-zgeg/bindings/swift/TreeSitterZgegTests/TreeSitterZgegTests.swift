import XCTest
import SwiftTreeSitter
import TreeSitterZgeg

final class TreeSitterZgegTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_zgeg())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Zgeg grammar")
    }
}
