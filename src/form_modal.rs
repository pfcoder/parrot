use actix_web::web;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

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
				"医院名称": self.hospital_name,
				"城市": self.city,
				"试用者": self.user,
				"申请人姓名": self.applicant,
				"申请人联系方式": self.contact,
				"产品": self.prod,
				"型号": self.model,
				"首选时间 月": self.month,
				"首选时间 日": self.day,
				"备选时间 月": self.bmonth,
				"备选时间 日": self.bday,
		})
	}
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct BuyData {
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

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RegisterDataPs {
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

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RegisterDataBu {
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

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RepairData {
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
			Forms::Trail(data) => vec![
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
			Forms::Buy(data) => vec![
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
			Forms::RegisterPs(data) => vec![
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

			Forms::RegisterBu(data) => vec![
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

			Forms::Repair(data) => vec![
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
