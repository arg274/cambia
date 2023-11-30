import type { SegmentDetails } from "./SegmentDetails";

export interface LaneDetails {
    radius: number,
    segmentSize: number,
    segmentGap: number,
    segments: SegmentDetails[],
}