use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::product_reviews::ReviewStatus;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReviewCreate {
    product_id: i32,
    status: Option<ReviewStatus>,
    reviewer: Option<String>,
    reviewer_email: String,
    review: Option<String>,
    rating: Option<i32>,
    verified: Option<bool>,
}
#[derive(Default)]
pub struct WithId(i32);
#[derive(Default)]
pub struct NoId;
#[derive(Default)]
pub struct WithEmail(String);
#[derive(Default)]
pub struct NoEmail;
#[derive(Default)]
pub struct ProductReviewCreateBuilder<I, E> {
    product_id: I,
    status: Option<ReviewStatus>,
    reviewer: Option<String>,
    reviewer_email: E,
    review: Option<String>,
    rating: Option<i32>,
    verified: Option<bool>,
}
impl<I, E> ProductReviewCreateBuilder<I, E> {
    /// Unique identifier for the product that the review belongs to.
    pub fn product_id(self, id: i32) -> ProductReviewCreateBuilder<WithId, E> {
        ProductReviewCreateBuilder {
            product_id: WithId(id),
            status: self.status,
            reviewer: self.reviewer,
            reviewer_email: self.reviewer_email,
            review: self.review,
            rating: self.rating,
            verified: self.verified,
        }
    }
    /// Status of the review. Options: approved, hold, spam, unspam, trash and untrash. Defaults to approved.
    pub fn status(mut self, status: ReviewStatus) -> Self {
        let _ = self.status.insert(status);
        self
    }
    /// Reviewer name.
    pub fn reviewer(mut self, reviewer: impl Into<String>) -> Self {
        let _ = self.reviewer.insert(reviewer.into());
        self
    }
    /// Reviewer email.
    pub fn reviewer_email(
        self,
        reviewer_email: impl Into<String>,
    ) -> ProductReviewCreateBuilder<I, WithEmail> {
        ProductReviewCreateBuilder {
            product_id: self.product_id,
            status: self.status,
            reviewer: self.reviewer,
            reviewer_email: WithEmail(reviewer_email.into()),
            review: self.review,
            rating: self.rating,
            verified: self.verified,
        }
    }
    /// The content of the review.
    pub fn review(mut self, review: impl Into<String>) -> Self {
        let _ = self.review.insert(review.into());
        self
    }
    /// Review rating (0 to 5).
    pub fn rating(mut self, rating: i32) -> Self {
        let rating = if rating >= 5 {
            5
        } else if rating <= 0 {
            0
        } else {
            rating
        };
        let _ = self.rating.insert(rating);
        self
    }
    /// Shows if the reviewer bought the product or not.
    pub fn verified(mut self, verified: bool) -> Self {
        let _ = self.verified.insert(verified);
        self
    }
}
impl ProductReviewCreateBuilder<WithId, WithEmail> {
    pub fn build(self) -> ProductReviewCreate {
        ProductReviewCreate {
            product_id: self.product_id.0,
            status: self.status,
            reviewer: self.reviewer,
            reviewer_email: self.reviewer_email.0,
            review: self.review,
            rating: self.rating,
            verified: self.verified,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReviewUpdate {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Unique identifier for the product that the review belongs to.
    pub product_id: Option<i32>,
    /// Status of the review. Options: approved, hold, spam, unspam, trash and untrash. Defaults to approved.
    pub status: Option<ReviewStatus>,
    /// Reviewer name.
    pub reviewer: Option<String>,
    /// Reviewer email.
    pub reviewer_email: Option<String>,
    /// The content of the review.
    pub review: Option<String>,
    /// Review rating (0 to 5).
    pub rating: Option<i32>,
    /// Shows if the reviewer bought the product or not.
    pub verified: Option<bool>,
}
#[derive(Default)]
pub struct ProductReviewUpdateBuilder {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Unique identifier for the product that the review belongs to.
    pub product_id: Option<i32>,
    /// Status of the review. Options: approved, hold, spam, unspam, trash and untrash. Defaults to approved.
    pub status: Option<ReviewStatus>,
    /// Reviewer name.
    pub reviewer: Option<String>,
    /// Reviewer email.
    pub reviewer_email: Option<String>,
    /// The content of the review.
    pub review: Option<String>,
    /// Review rating (0 to 5).
    pub rating: Option<i32>,
    /// Shows if the reviewer bought the product or not.
    pub verified: Option<bool>,
}
impl ProductReviewUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Unique identifier for the product that the review belongs to.
    pub fn product_id(mut self, product_id: i32) -> Self {
        let _ = self.product_id.insert(product_id);
        self
    }
    /// Status of the review. Options: approved, hold, spam, unspam, trash and untrash. Defaults to approved.
    pub fn status(mut self, status: ReviewStatus) -> Self {
        let _ = self.status.insert(status);
        self
    }
    /// Reviewer name.
    pub fn reviewer(mut self, reviewer: impl Into<String>) -> Self {
        let _ = self.reviewer.insert(reviewer.into());
        self
    }
    /// Reviewer email.
    pub fn reviewer_email(mut self, reviewer_email: impl Into<String>) -> Self {
        let _ = self.reviewer_email.insert(reviewer_email.into());
        self
    }
    /// The content of the review.
    pub fn review(mut self, review: impl Into<String>) -> Self {
        let _ = self.review.insert(review.into());
        self
    }
    /// Review rating (0 to 5).
    pub fn rating(mut self, rating: i32) -> Self {
        let rating = if rating >= 5 {
            5
        } else if rating <= 0 {
            0
        } else {
            rating
        };
        let _ = self.rating.insert(rating);
        self
    }
    /// Shows if the reviewer bought the product or not.
    pub fn verified(mut self, verified: bool) -> Self {
        let _ = self.verified.insert(verified);
        self
    }
    pub fn build(self) -> ProductReviewUpdate {
        ProductReviewUpdate {
            id: self.id,
            product_id: self.product_id,
            status: self.status,
            reviewer: self.reviewer,
            reviewer_email: self.reviewer_email,
            review: self.review,
            rating: self.rating,
            verified: self.verified,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        product_reviews::{ProductReview, ReviewStatus},
        ApiClient, BatchObject, Entity,
    };

    #[tokio::test]
    async fn test_list_all_product_review() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<ProductReview>(Entity::ProductReview)
            .await
            .unwrap();
        assert!(result.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_product_review() {
        let _client = ApiClient::from_env().unwrap();
    }
    #[tokio::test]
    async fn test_create_update_batch_update_delete_product_review() {
        let client = ApiClient::from_env().unwrap();
        let create = ProductReview::create()
            .product_id(3982)
            .reviewer_email("president@world.ua")
            .review("Cool test review")
            .rating(5)
            .status(ReviewStatus::Approved)
            .reviewer("Zorro")
            .verified(true)
            .build();
        let created: ProductReview = client.create(Entity::ProductReview, create).await.unwrap();
        assert_eq!(created.reviewer, "Zorro");
        let update = ProductReview::update().rating(4).build();
        let update_to_batch = ProductReview::update().rating(5).id(created.id).build();
        let batch = BatchObject::builder().add_update(update_to_batch).build();
        let updated: ProductReview = client
            .update(Entity::ProductReview, created.id, update)
            .await
            .unwrap();
        assert_eq!(updated.rating, 4);
        let batch_updated: BatchObject<ProductReview> = client
            .batch_update(Entity::ProductReview, batch)
            .await
            .unwrap();
        assert!(batch_updated.update.is_some());
        let _deleted: serde_json::Value = client
            .delete(Entity::ProductReview, created.id)
            .await
            .unwrap();
    }
}
