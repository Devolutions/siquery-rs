Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_hotfixes table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_hotfixes = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_hotfixes' }
				$wmi_hotfixes | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_hotfixes = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_hotfixes' }
				$schema = Get-Content $wmi_hotfixes.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_hotfixes = Get-Siq 'wmi_hotfixes' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_hotfixes' }).FullName -Raw

				$json = $wmi_hotfixes | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
