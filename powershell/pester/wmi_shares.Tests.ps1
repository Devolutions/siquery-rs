Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_shares table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_shares = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_shares' }
				$wmi_shares | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_shares = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_shares' }
				$schema = Get-Content $wmi_shares.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_shares = Get-Siq 'wmi_shares' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_shares' }).FullName -Raw

				$json = $wmi_shares | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
