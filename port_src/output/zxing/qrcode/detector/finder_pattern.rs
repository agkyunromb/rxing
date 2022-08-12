/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::google::zxing::qrcode::detector;

/**
 * <p>Encapsulates a finder pattern, which are the three square patterns found in
 * the corners of QR Codes. It also encapsulates a count of similar finder patterns,
 * as a convenience to the finder's bookkeeping.</p>
 *
 * @author Sean Owen
 */
pub struct FinderPattern {
    super: ResultPoint;

     let estimated_module_size: f32;

     let count: i32;
}

impl FinderPattern {

    fn new( pos_x: f32,  pos_y: f32,  estimated_module_size: f32) -> FinderPattern {
        this(pos_x, pos_y, estimated_module_size, 1);
    }

    fn new( pos_x: f32,  pos_y: f32,  estimated_module_size: f32,  count: i32) -> FinderPattern {
        super(pos_x, pos_y);
        let .estimatedModuleSize = estimated_module_size;
        let .count = count;
    }

    pub fn  get_estimated_module_size(&self) -> f32  {
        return self.estimated_module_size;
    }

    pub fn  get_count(&self) -> i32  {
        return self.count;
    }

    /**
   * <p>Determines if this finder pattern "about equals" a finder pattern at the stated
   * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
   */
    fn  about_equals(&self,  module_size: f32,  i: f32,  j: f32) -> bool  {
        if Math::abs(i - get_y()) <= module_size && Math::abs(j - get_x()) <= module_size {
             let module_size_diff: f32 = Math::abs(module_size - self.estimated_module_size);
            return module_size_diff <= 1.0f || module_size_diff <= self.estimated_module_size;
        }
        return false;
    }

    /**
   * Combines this object's current estimate of a finder pattern position and module size
   * with a new estimate. It returns a new {@code FinderPattern} containing a weighted average
   * based on count.
   */
    fn  combine_estimate(&self,  i: f32,  j: f32,  new_module_size: f32) -> FinderPattern  {
         let combined_count: i32 = self.count + 1;
         let combined_x: f32 = (self.count * get_x() + j) / combined_count;
         let combined_y: f32 = (self.count * get_y() + i) / combined_count;
         let combined_module_size: f32 = (self.count * self.estimated_module_size + new_module_size) / combined_count;
        return FinderPattern::new(combined_x, combined_y, combined_module_size, combined_count);
    }
}

