Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_local_accounts table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_local_accounts = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_local_accounts' }
				$wmi_local_accounts | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_local_accounts = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_local_accounts' }
				$schema = Get-Content $wmi_local_accounts.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_local_accounts = Get-Siq 'wmi_local_accounts' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_local_accounts' }).FullName -Raw

				$json = $wmi_local_accounts | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
