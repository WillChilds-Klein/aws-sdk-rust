// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSipRule`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sip_rule_id(impl Into<String>)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::sip_rule_id) / [`set_sip_rule_id(Option<String>)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::set_sip_rule_id):<br>required: **true**<br><p>The SIP rule ID.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::set_name):<br>required: **true**<br><p>The new name for the specified SIP rule.</p><br>
    ///   - [`disabled(bool)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::disabled) / [`set_disabled(Option<bool>)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::set_disabled):<br>required: **false**<br><p>The new value that indicates whether the rule is disabled.</p><br>
    ///   - [`target_applications(SipRuleTargetApplication)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::target_applications) / [`set_target_applications(Option<Vec::<SipRuleTargetApplication>>)`](crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::set_target_applications):<br>required: **false**<br><p>The new list of target applications.</p><br>
    /// - On success, responds with [`UpdateSipRuleOutput`](crate::operation::update_sip_rule::UpdateSipRuleOutput) with field(s):
    ///   - [`sip_rule(Option<SipRule>)`](crate::operation::update_sip_rule::UpdateSipRuleOutput::sip_rule): <p>The updated SIP rule details.</p>
    /// - On failure, responds with [`SdkError<UpdateSipRuleError>`](crate::operation::update_sip_rule::UpdateSipRuleError)
    pub fn update_sip_rule(&self) -> crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder {
        crate::operation::update_sip_rule::builders::UpdateSipRuleFluentBuilder::new(self.handle.clone())
    }
}
