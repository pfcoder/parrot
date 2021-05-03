use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum Forms {
	Trail(TrailData),
	Buy(BuyData),
	RegisterPs(RegisterDataPs),
	RegisterBu(RegisterDataBu),
	Repair(RepairData),
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TrailData {
	#[serde(default)]
	#[serde(skip_serializing)]
	pub token: String,

	#[serde(default)]
	pub form_type: String,

	#[serde(default)]
	pub form_title: String,

	// trail
	#[serde(default)]
	pub hospital_name: String,

	#[serde(default)]
	pub applicant: String,

	#[serde(default)]
	pub city: String,

	#[serde(default)]
	pub user: String,

	#[serde(default)]
	pub contact: String,

	#[serde(default)]
	pub prod: String,

	#[serde(default)]
	pub model: String,

	#[serde(default)]
	pub month: String,

	#[serde(default)]
	pub day: String,

	#[serde(default)]
	pub bmonth: String,

	#[serde(default)]
	pub bday: String,
}

impl TrailData {
	pub fn get_json_maps(&self) -> Value {
		json!({
				"title": "试用申请",
				"kvs": [
					{"key": "医院名称", "value": self.hospital_name},
					{"key": "城市", "value": self.city},
					{"key": "试用者", "value": self.user},
					{"key": "申请人姓名", "value": self.applicant},
					{"key": "申请人联系方式", "value": self.contact},
					{"key": "产品", "value": self.prod},
					{"key": "型号", "value": self.model},
					{"key": "首选时间 月", "value": self.month},
					{"key": "首选时间 日", "value": self.day},
					{"key": "备选时间 月", "value": self.bmonth},
					{"key": "备选时间 日", "value": self.bday},
				]
		})
	}
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct BuyData {
	#[serde(default)]
	#[serde(skip_serializing)]
	pub token: String,

	#[serde(default)]
	pub form_type: String,

	#[serde(default)]
	pub form_title: String,

	#[serde(default)]
	pub per_com: String,

	#[serde(default)]
	pub company: String,

	#[serde(default)]
	pub city: String,

	#[serde(default)]
	pub name: String,

	#[serde(default)]
	pub contact: String,

	#[serde(default)]
	pub prod: String,

	#[serde(default)]
	pub model: String,

	#[serde(default)]
	pub amount: String,
}

impl BuyData {
	pub fn get_json_maps(&self) -> Value {
		json!({
				"title": "我想购买",
				"kvs": [
					{"key": "单位还是个人购买", "value": self.per_com},
					{"key": "单位名称", "value": self.company},
					{"key": "单位所在城市", "value": self.city},
					{"key": "姓名", "value": self.name},
					{"key": "联系方式", "value": self.contact},
					{"key": "产品", "value": self.prod},
					{"key": "型号", "value": self.model},
					{"key": "数量", "value": self.amount},
				]
		})
	}
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RegisterDataPs {
	#[serde(default)]
	#[serde(skip_serializing)]
	pub token: String,

	#[serde(default)]
	pub form_type: String,

	#[serde(default)]
	pub form_title: String,

	#[serde(default)]
	pub username: String,

	#[serde(default)]
	pub password: String,

	#[serde(default)]
	pub passwordq: String,

	#[serde(default)]
	pub passworda: String,

	#[serde(default)]
	pub usertype: String,

	#[serde(default)]
	pub name: String,

	#[serde(default)]
	pub year: String,

	#[serde(default)]
	pub month: String,

	#[serde(default)]
	pub company: String,

	#[serde(default)]
	pub title: String,

	#[serde(default)]
	pub expertise: String,

	#[serde(default)]
	pub cell: String,

	#[serde(default)]
	pub id: String,

	#[serde(default)]
	pub recommander: String,

	#[serde(default)]
	pub recommander_name: String,
}

impl RegisterDataPs {
	pub fn get_json_maps(&self) -> Value {
		json!({
				"title": "豪友荟个人会员注册",
				"kvs": [
					{"key": "邮箱", "value": self.username},
					{"key": "密码", "value": self.password},
					{"key": "密码提示问题", "value": self.passwordq},
					{"key": "密码提示答案", "value": self.passworda},
					{"key": "姓名", "value": self.name},
					{"key": "生日 年", "value": self.year},
					{"key": "生日 月", "value": self.month},
					{"key": "工作单位", "value": self.company},
					{"key": "职称", "value": self.title},
					{"key": "专业", "value": self.expertise},
					{"key": "手机", "value": self.cell},
					{"key": "身份证号后4位", "value": self.id},
					{"key": "推荐人", "value": self.recommander},
					{"key": "推荐人登录名", "value": self.recommander_name},
				]
		})
	}
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RegisterDataBu {
	#[serde(default)]
	#[serde(skip_serializing)]
	pub token: String,

	#[serde(default)]
	pub form_type: String,

	#[serde(default)]
	pub form_title: String,

	#[serde(default)]
	pub username: String,

	#[serde(default)]
	pub password: String,

	#[serde(default)]
	pub passwordq: String,

	#[serde(default)]
	pub passworda: String,

	#[serde(default)]
	pub usertype: String,

	#[serde(default)]
	pub name: String,

	#[serde(default)]
	pub address: String,

	#[serde(default)]
	pub postcode: String,

	#[serde(default)]
	pub personincharge: String,

	#[serde(default)]
	pub tel: String,

	#[serde(default)]
	pub cell: String,

	#[serde(default)]
	pub province: String,

	#[serde(default)]
	pub city: String,

	#[serde(default)]
	pub mainbusiness: String,

	#[serde(default)]
	pub sellingbrand: String,

	#[serde(default)]
	pub recommander: String,

	#[serde(default)]
	pub recommander_name: String,

	#[serde(default)]
	pub subcompany: String,

	#[serde(default)]
	pub subcompany_name: String,
}

impl RegisterDataBu {
	pub fn get_json_maps(&self) -> Value {
		json!({
				"title": "豪友荟商业会员注册信息表",
				"kvs": [
					{"key": "邮箱", "value": self.username},
					{"key": "密码", "value": self.password},
					{"key": "密码提示问题", "value": self.passwordq},
					{"key": "密码提示答案", "value": self.passworda},
					{"key": "企业名称", "value": self.name},
					{"key": "地址", "value": self.address},
					{"key": "邮编", "value": self.postcode},
					{"key": "负责人", "value": self.personincharge},
					{"key": "联系电话", "value": self.tel},
					{"key": "负责人手机", "value": self.cell},
					{"key": "拟销售区域 省", "value": self.province},
					{"key": "拟销售区域 市/县", "value": self.city},
					{"key": "公司主营业务", "value": self.mainbusiness},
					{"key": "公司常年经销的眼科品牌", "value": self.sellingbrand},
					{"key": "推荐人", "value": self.recommander},
					{"key": "推荐人登录名或公司全称", "value": self.recommander_name},
					{"key": "下属关联公司", "value": self.subcompany},
					{"key": "下属关联公司名称", "value": self.subcompany_name},
				]
		})
	}
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RepairData {
	#[serde(default)]
	#[serde(skip_serializing)]
	pub token: String,

	#[serde(default)]
	pub form_type: String,

	#[serde(default)]
	pub form_title: String,

	#[serde(default)]
	pub hospital_name: String,

	#[serde(default)]
	pub city: String,

	#[serde(default)]
	pub name: String,

	#[serde(default)]
	pub contact: String,

	#[serde(default)]
	pub prod: String,

	#[serde(default)]
	pub model: String,

	#[serde(default)]
	pub serial: String,

	#[serde(default)]
	pub year: String,

	#[serde(default)]
	pub month: String,

	#[serde(default)]
	pub question: String,
}

impl RepairData {
	pub fn get_json_maps(&self) -> Value {
		json!({
				"title": "报修申请",
				"kvs": [
					{"key": "医院名称", "value": self.hospital_name},
					{"key": "医院所在城市", "value": self.city},
					{"key": "联系人", "value": self.name},
					{"key": "联系方式", "value": self.contact},
					{"key": "产品", "value": self.prod},
					{"key": "型号", "value": self.model},
					{"key": "序列号", "value": self.serial},
					{"key": "购买时间 年", "value": self.year},
					{"key": "购买时间 月", "value": self.month},
					{"key": "问题描述", "value": self.question},
				]
		})
	}
}

impl Forms {
	pub fn get_type(&self) -> &str {
		match self {
			Forms::Trail(data) => &data.form_type,
			Forms::Buy(data) => &data.form_type,
			Forms::RegisterPs(data) => &data.form_type,
			Forms::RegisterBu(data) => &data.form_type,
			Forms::Repair(data) => &data.form_type,
		}
	}

	pub fn get_header(&self) -> Vec<String> {
		match self {
			Forms::Trail(_data) => vec![
				"类型".to_string(),
				"标题".to_string(),
				"医院名称".to_string(),
				"城市".to_string(),
				"试用者".to_string(),
				"申请人姓名".to_string(),
				"申请人联系方式".to_string(),
				"产品".to_string(),
				"型号".to_string(),
				"首选时间 月".to_string(),
				"首选时间 日".to_string(),
				"备选时间 月".to_string(),
				"备选时间 日".to_string(),
			],
			Forms::Buy(_data) => vec![
				"类型".to_string(),
				"标题".to_string(),
				"单位还是个人购买".to_string(),
				"单位名称".to_string(),
				"单位所在城市".to_string(),
				"姓名".to_string(),
				"联系方式".to_string(),
				"产品".to_string(),
				"型号".to_string(),
				"数量".to_string(),
			],
			Forms::RegisterPs(_data) => vec![
				"类型".to_string(),
				"标题".to_string(),
				"邮箱(作为登录名)".to_string(),
				"密码".to_string(),
				"密码提示问题".to_string(),
				"密码提示答案".to_string(),
				"用户类型".to_string(),
				"姓名".to_string(),
				"生日 年".to_string(),
				"生日 月".to_string(),
				"工作单位".to_string(),
				"职称".to_string(),
				"专业".to_string(),
				"手机".to_string(),
				"身份证号后4位".to_string(),
				"推荐人".to_string(),
				"推荐人登录名".to_string(),
			],

			Forms::RegisterBu(_data) => vec![
				"类型".to_string(),
				"标题".to_string(),
				"邮箱(作为登录名)".to_string(),
				"密码".to_string(),
				"密码提示问题".to_string(),
				"密码提示答案".to_string(),
				"用户类型".to_string(),
				"企业名称".to_string(),
				"地址".to_string(),
				"邮编".to_string(),
				"负责人".to_string(),
				"联系电话".to_string(),
				"负责人手机".to_string(),
				"拟销售区域 省".to_string(),
				"拟销售区域 市/县".to_string(),
				"公司主营业务".to_string(),
				"公司常年经销的眼科品牌".to_string(),
				"推荐人".to_string(),
				"推荐人登录名或公司全称".to_string(),
				"下属关联公司".to_string(),
				"下属关联公司名称".to_string(),
			],

			Forms::Repair(_data) => vec![
				"类型".to_string(),
				"标题".to_string(),
				"医院名称".to_string(),
				"医院所在城市".to_string(),
				"联系人".to_string(),
				"联系方式".to_string(),
				"产品".to_string(),
				"型号".to_string(),
				"序列号".to_string(),
				"购买时间 年".to_string(),
				"购买时间 月".to_string(),
				"问题描述".to_string(),
			],
		}
	}
}
