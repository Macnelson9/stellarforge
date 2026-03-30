import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { TutorAccountSettingsModule } from './tutor-account-settings/tutor-account-settings.module';
import { StudentAccountSettingsModule } from './student-account-settings/student-account-settings.module';
import { AdminModeratorAccountSettingsModule } from './admin-moderator-account-settings/admin-moderator-account-settings.module';
import { AdminAuthModule } from './admin-auth/admin-auth.module';
import { CoursePerformanceLeaderboardModule } from './course-performance-leaderboard/course-performance-leaderboard.module';
import { CourseCategorizationFilteringModule } from './course-categorization-filtering/course-categorization-filtering.module';
import { PrivateTutoringBookingsModule } from './private-tutoring-bookings/private-tutoring-bookings.module';
import { GamificationPointsModule } from './gamification-points/gamification-points.module';
import { CourseCertificationNftAchievementsModule } from './course-certification-nft-achievements/course-certification-nft-achievements.module';
import { CourseRatingsFeedbackModule } from './course-ratings-feedback/course-ratings-feedback.module';
import { TutorJwtAuthModule } from './tutor-jwt-auth/tutor-jwt-auth.module';
import { StudentWishlistModule } from './student-wishlist/student-wishlist.module';
import { FaqManagementModule } from './faq-management/faq-management.module';
import { TermsConditionsManagementModule } from './terms-conditions-management/terms-conditions-management.module';
import { PrivacyPolicyManagementModule } from './privacy-policy-management/privacy-policy-management.module';
import { TutorReportsAnalyticsModule } from './tutor-reports-analytics/tutor-reports-analytics.module';
import { StudentReportsAnalyticsModule } from './student-reports-analytics/student-reports-analytics.module';
import { CourseReportsAnalyticsModule } from './course-reports-analytics/course-reports-analytics.module';
import { CertificateSocialSharingModule } from './certificate-social-sharing/certificate-social-sharing.module';
import { CertificateDownloadModule } from './certificate-download/certificate-download.module';
import { AdminCertificateNameChangeReviewModule } from './admin-certificate-name-change-review/admin-certificate-name-change-review.module';
import { StudentCertificateNameChangeRequestModule } from './student-certificate-name-change-request/student-certificate-name-change-request.module';
import { AdminFinancialAidManagementModule } from './admin-financial-aid-management/admin-financial-aid-management.module';
import { StudentFinancialAidApplicationModule } from './student-financial-aid-application/student-financial-aid-application.module';
import { AboutManagementModule } from './about-management/about-management.module';
import { ContactMessageModule } from './contact-message/contact-message.module';
import { BadgesNftModule } from './badges-nft/badges-nft.module';
import { NotificationSystemModule } from './notification-system/notification-system.module';
import { OrganizationManagementModule } from './organization-management/organization-management.module';
import { OrganizationMembersModule } from './organization-members/organization-members.module';
import { PointsManagementModule } from './points-management/points-management.module';
import { RemovalRequestModule } from './removal-request/removal-request.module';
import { ReportAbuseModule } from './report-abuse/report-abuse.module';
import { SessionManagementModule } from './session-management/session-management.module';
import { SubscriptionPlanManagementModule } from './subscription-plan-management/subscription-plan-management.module';
import { StudentAuthModule } from './student-auth/student-auth.module';
import { StudentCartModule } from './student-cart/student-cart.module';
import { GoogleAuthModule } from './google-auth/google-auth.module';
import { StudentSavedCoursesModule } from './student-saved-courses/student-saved-courses.module';
import { AdminCourseManagementModule } from './admin-course-management/admin-course-management.module';

@Module({
  imports: [
    TutorAccountSettingsModule,
    StudentAccountSettingsModule,
    AdminModeratorAccountSettingsModule,
    AdminAuthModule,
    CoursePerformanceLeaderboardModule,
    CourseCategorizationFilteringModule,
    PrivateTutoringBookingsModule,
    GamificationPointsModule,
    CourseCertificationNftAchievementsModule,
    CourseRatingsFeedbackModule,
    TutorJwtAuthModule,
    StudentWishlistModule,
    FaqManagementModule,
    TermsConditionsManagementModule,
    PrivacyPolicyManagementModule,
    TutorReportsAnalyticsModule,
    StudentReportsAnalyticsModule,
    CourseReportsAnalyticsModule,
    CertificateSocialSharingModule,
    CertificateDownloadModule,
    AdminCertificateNameChangeReviewModule,
    StudentCertificateNameChangeRequestModule,
    AdminFinancialAidManagementModule,
    StudentFinancialAidApplicationModule,
    AboutManagementModule,
    ContactMessageModule,
    BadgesNftModule,
    NotificationSystemModule,
    OrganizationManagementModule,
    OrganizationMembersModule,
    PointsManagementModule,
    RemovalRequestModule,
    ReportAbuseModule,
    SessionManagementModule,
    SubscriptionPlanManagementModule,
    StudentAuthModule,
    StudentCartModule,
    GoogleAuthModule,
    StudentSavedCoursesModule,
    AdminCourseManagementModule,
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
