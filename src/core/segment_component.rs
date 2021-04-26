use std::slice;

/// Enum describing each component of a tantivy segment.
/// Each component is stored in its own file,
/// using the pattern `segment_uuid`.`component_extension`,
/// except the delete component that takes an `segment_uuid`.`delete_opstamp`.`component_extension`
#[derive(Copy, Clone)]
pub enum SegmentComponent {
    /// Postings (or inverted list). Sorted lists of document ids, associated to terms
    Postings,
    /// Positions of terms in each document.
    Positions,
    /// Index to seek within the position file
    PositionsSkip,
    /// Column-oriented random-access storage of fields.
    FastFields,
    /// Stores the sum  of the length (in terms) of each field for each document.
    /// Field norms are stored as a special u64 fast field.
    FieldNorms,
    /// Dictionary associating `Term`s to `TermInfo`s which is
    /// simply an address into the `postings` file and the `positions` file.
    Terms,
    /// Row-oriented, compressed storage of the documents.
    /// Accessing a document from the store is relatively slow, as it
    /// requires to decompress the entire block it belongs to.
    Store,
    /// Bitset describing which document of the segment is deleted.
    Delete,
}

impl SegmentComponent {
    /// Iterates through the components.
    pub fn iterator() -> slice::Iter<'static, SegmentComponent> {
        static SEGMENT_COMPONENTS: [SegmentComponent; 8] = [
            SegmentComponent::Postings,
            SegmentComponent::Positions,
            SegmentComponent::PositionsSkip,
            SegmentComponent::FastFields,
            SegmentComponent::FieldNorms,
            SegmentComponent::Terms,
            SegmentComponent::Store,
            SegmentComponent::Delete,
        ];
        SEGMENT_COMPONENTS.iter()
    }
}
